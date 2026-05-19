//! Solar test runner.
//!
//! This crate is invoked in `crates/solar/tests.rs` with the path to the `solar` binary.

#![allow(unreachable_pub)]

use eyre::{Context, Result, eyre};
use std::{collections::BTreeMap, fmt::Write as _, fs, path::Path};
use ui_test::{color_eyre::eyre, spanned::Spanned};

mod errors;
mod solc;
mod utils;

/// Runs all the tests with the given `solar` command path.
pub fn run_tests(cmd: &'static Path) -> Result<()> {
    ui_test::color_eyre::install()?;

    let mut args = ui_test::Args::test()?;

    // Fast path for `--list`, invoked by `cargo-nextest`.
    {
        let mut dummy_config = ui_test::Config::dummy();
        dummy_config.with_args(&args);
        if ui_test::nextest::emulate(&mut vec![dummy_config]) {
            return Ok(());
        }
    }

    // Condense output if not explicitly requested.
    let requested_pretty = || std::env::args().any(|x| x.contains("--format"));
    if matches!(args.format, ui_test::Format::Pretty) && !requested_pretty() {
        args.format = ui_test::Format::Terse;
    }

    let mut modes = &[Mode::Ui, Mode::SolcSolidity, Mode::SolcYul][..];
    let mode_tmp;
    if let Ok(mode) = std::env::var("TESTER_MODE") {
        mode_tmp = Mode::parse(&mode).ok_or_else(|| eyre!("invalid mode: {mode}"))?;
        modes = std::slice::from_ref(&mode_tmp);
    }

    let accounting_modes = modes.iter().copied().filter(|mode| mode.is_solc()).collect::<Vec<_>>();
    if !accounting_modes.is_empty() {
        emit_corpus_accounting(&accounting_modes)?;
    }

    let tmp_dir = tempfile::tempdir()?;
    let tmp_dir = &*Box::leak(tmp_dir.path().to_path_buf().into_boxed_path());
    for &mode in modes {
        let cfg = MyConfig::<'static> { mode, tmp_dir };
        let config = config(cmd, &args, mode);

        let text_emitter: Box<dyn ui_test::status_emitter::StatusEmitter> = args.format.into();
        let gha_emitter = ui_test::status_emitter::Gha { name: mode.to_string(), group: true };
        let status_emitter = (text_emitter, gha_emitter);

        ui_test::run_tests_generic(
            vec![config],
            move |path, config| file_filter(path, config, cfg),
            move |config, contents| per_file_config(config, contents, cfg),
            status_emitter,
        )?;
    }

    Ok(())
}

fn config(cmd: &'static Path, args: &ui_test::Args, mode: Mode) -> ui_test::Config {
    let root = repo_root();

    let path = mode.corpus_path();
    let tests_root = root.join(path);
    assert!(
        tests_root.exists(),
        "tests root directory does not exist: {path};\n\
         you may need to initialize submodules: `git submodule update --init --checkout`"
    );

    let mut config = ui_test::Config {
        // `host` and `target` are used for `//@ignore-...` comments.
        host: Some(get_host().to_string()),
        target: None,
        root_dir: tests_root,
        program: ui_test::CommandBuilder {
            program: cmd.into(),
            args: {
                let mut args =
                    vec!["-j1", "--error-format=rustc-json", "-Zui-testing", "-Zparse-yul"];
                if mode.is_solc() {
                    args.push("--stop-after=parsing");
                }
                args.into_iter().map(Into::into).collect()
            },
            out_dir_flag: None,
            input_file_flag: None,
            envs: vec![],
            cfg_flag: None,
        },
        output_conflict_handling: ui_test::error_on_output_conflict,
        bless_command: Some("cargo uibless".into()),
        out_dir: root.join("target/ui"),
        comment_start: "//",
        diagnostic_extractor: ui_test::diagnostics::rustc::rustc_diagnostics_extractor,
        ..ui_test::Config::dummy()
    };

    macro_rules! register_custom_flags {
        ($($ty:ty),* $(,)?) => {
            $(
                config.custom_comments.insert(<$ty>::NAME, <$ty>::parse);
                if let Some(default) = <$ty>::DEFAULT {
                    config.comment_defaults.base().add_custom(<$ty>::NAME, default);
                }
            )*
        };
    }
    register_custom_flags![];

    config.comment_defaults.base().exit_status = None.into();
    config.comment_defaults.base().require_annotations = Spanned::dummy(true).into();
    config.comment_defaults.base().require_annotations_for_level =
        Spanned::dummy(ui_test::diagnostics::Level::Warn).into();

    let filters = [
        (ui_test::Match::PathBackslash, b"/".to_vec()),
        #[cfg(windows)]
        (ui_test::Match::Exact(vec![b'\r']), b"".to_vec()),
        #[cfg(windows)]
        (ui_test::Match::Exact(br"\\?\".to_vec()), b"".to_vec()),
        (root.into(), b"ROOT".to_vec()),
    ];
    config.comment_defaults.base().normalize_stderr.extend(filters.iter().cloned());
    config.comment_defaults.base().normalize_stdout.extend(filters);

    let filters: &[(&str, &str)] = &[
        // Erase line and column info.
        (r"\.(\w+):[0-9]+:[0-9]+(: [0-9]+:[0-9]+)?", ".$1:LL:CC"),
    ];
    for &(pattern, replacement) in filters {
        config.filter(pattern, replacement);
    }
    let stdout_filters: &[(&str, &str)] = &[
        //
        (&env!("CARGO_PKG_VERSION").replace(".", r"\."), "VERSION"),
    ];
    for &(pattern, replacement) in stdout_filters {
        config.stdout_filter(pattern, replacement);
    }
    let stderr_filters: &[(&str, &str)] = &[];
    for &(pattern, replacement) in stderr_filters {
        config.stderr_filter(pattern, replacement);
    }

    config.with_args(args);

    if mode.is_solc() {
        // Override `bless` handler, since we don't want to write Solc tests.
        config.output_conflict_handling = ui_test::ignore_output_conflict;
        // Skip parsing comments since they result in false positives.
        config.comment_start = "\0";
        config.comment_defaults.base().require_annotations = Spanned::dummy(false).into();
    }

    config
}

fn get_host() -> &'static str {
    static CACHE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| {
        let mut config = ui_test::Config::dummy();
        config.program = ui_test::CommandBuilder::rustc();
        config.fill_host_and_target().unwrap();
        config.host.unwrap()
    })
}

fn file_filter(path: &Path, config: &ui_test::Config, cfg: MyConfig<'_>) -> Option<bool> {
    path.extension().filter(|&ext| ext == "sol" || (cfg.mode.allows_yul() && ext == "yul"))?;
    if !ui_test::default_any_file_filter(path, config) {
        return Some(false);
    }
    let skip = match cfg.mode {
        Mode::Ui => false,
        Mode::SolcSolidity => solc::solidity::should_skip(path).is_err(),
        Mode::SolcYul => solc::yul::should_skip(path).is_err(),
    };
    Some(!skip)
}

fn per_file_config(config: &mut ui_test::Config, file: &Spanned<Vec<u8>>, cfg: MyConfig<'_>) {
    let Ok(src) = std::str::from_utf8(&file.content) else {
        return;
    };
    let path = file.span.file.as_path();

    if cfg.mode.is_solc() {
        return solc_per_file_config(config, src, path, cfg);
    }

    assert_eq!(config.comment_start, "//");
    let has_annotations = src.contains("//~");
    // TODO: https://github.com/oli-obk/ui_test/issues/341
    let is_check_fail = src.contains("check-fail");
    config.comment_defaults.base().require_annotations =
        Spanned::dummy(is_check_fail || has_annotations).into();
    let code = if is_check_fail || (has_annotations && src.contains("ERROR:")) { 1 } else { 0 };
    config.comment_defaults.base().exit_status = Spanned::dummy(code).into();
}

// For solc tests, we can't expect errors normally since we have different diagnostics.
// Instead, we check just the error code and ignore other output.
fn solc_per_file_config(config: &mut ui_test::Config, src: &str, path: &Path, cfg: MyConfig<'_>) {
    let code = match solc_exit_expectation(src) {
        SolcExitExpectation::Pass => Some(0),
        SolcExitExpectation::Fail => Some(1),
        SolcExitExpectation::Ignored => None,
    };
    config.comment_defaults.base().exit_status = code.map(Spanned::dummy).into();

    if matches!(cfg.mode, Mode::SolcSolidity) {
        let flags = &mut config.comment_defaults.base().compile_flags;
        let has_delimiters = solc::solidity::handle_delimiters(src, path, cfg.tmp_dir, |arg| {
            flags.push(arg.into_string().unwrap())
        });
        if has_delimiters {
            // HACK: skip the input file argument by using a dummy flag.
            config.program.input_file_flag = Some("-I".into());
        }
    }
}

#[derive(Clone, Copy)]
enum SolcExitExpectation {
    Pass,
    Fail,
    Ignored,
}

fn solc_exit_expectation(src: &str) -> SolcExitExpectation {
    let expected_errors = errors::Error::load_solc(src);
    let expected_error = expected_errors.iter().find(|e| e.is_error());
    if let Some(expected_error) = expected_error {
        // Expect failure only for parser errors. Other solc errors are not parser-corpus passes:
        // ui_test ignores their process status, and corpus accounting reports them separately.
        if expected_error.solc_kind.is_some_and(|kind| kind.is_parser_error()) {
            SolcExitExpectation::Fail
        } else {
            SolcExitExpectation::Ignored
        }
    } else {
        SolcExitExpectation::Pass
    }
}

fn emit_corpus_accounting(modes: &[Mode]) -> Result<()> {
    let root = repo_root();
    let mut accounting = CorpusAccounting::new(if modes.len() == 1 { modes[0].to_str() } else { "solc" });
    for &mode in modes {
        accounting.add_mode(collect_mode_accounting(root, mode));
    }

    let path = root.join(".pads-artifacts/tester-accounting.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&path, accounting.to_json())
        .with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap()
}

fn collect_mode_accounting(root: &Path, mode: Mode) -> ModeAccounting {
    let root_dir = root.join(mode.corpus_path());
    let mut accounting = ModeAccounting::new(mode, root_dir.strip_prefix(root).unwrap_or(&root_dir));

    if !root_dir.exists() {
        accounting.unavailable_prerequisites.push(UnavailablePrerequisite {
            path: accounting.root.clone(),
            reason: "corpus root does not exist; initialize submodules with `git submodule update --init --checkout`".into(),
        });
        return accounting;
    }

    let mut dirs = vec![root_dir];
    while let Some(dir) = dirs.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(err) => {
                accounting.unavailable_prerequisites.push(UnavailablePrerequisite {
                    path: relative_path(root, &dir),
                    reason: format!("failed to read directory: {err}"),
                });
                continue;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    accounting.unavailable_prerequisites.push(UnavailablePrerequisite {
                        path: relative_path(root, &dir),
                        reason: format!("failed to read directory entry: {err}"),
                    });
                    continue;
                }
            };
            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(file_type) => file_type,
                Err(err) => {
                    accounting.unavailable_prerequisites.push(UnavailablePrerequisite {
                        path: relative_path(root, &path),
                        reason: format!("failed to read file type: {err}"),
                    });
                    continue;
                }
            };
            if file_type.is_dir() {
                dirs.push(path);
                continue;
            }
            if !file_type.is_file() || !mode.is_candidate_file(&path) {
                continue;
            }

            accounting.discovered_files += 1;
            if let Some(reason) = mode.skip_reason(&path) {
                accounting.skipped_files += 1;
                *accounting.skipped_reasons.entry(reason).or_default() += 1;
                continue;
            }

            accounting.runnable_files += 1;
            let src = match fs::read_to_string(&path) {
                Ok(src) => src,
                Err(err) => {
                    accounting.unavailable_prerequisites.push(UnavailablePrerequisite {
                        path: relative_path(root, &path),
                        reason: format!("failed to read source: {err}"),
                    });
                    continue;
                }
            };
            match solc_exit_expectation(&src) {
                SolcExitExpectation::Pass => accounting.expected_passes += 1,
                SolcExitExpectation::Fail => accounting.expected_failures += 1,
                SolcExitExpectation::Ignored => accounting.ignored_exit_files += 1,
            }
        }
    }

    accounting
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).display().to_string()
}

#[derive(Default)]
struct CorpusAccounting {
    mode: &'static str,
    discovered_files: usize,
    runnable_files: usize,
    skipped_files: usize,
    expected_passes: usize,
    expected_failures: usize,
    ignored_exit_files: usize,
    unavailable_prerequisites: Vec<UnavailablePrerequisite>,
    modes: Vec<ModeAccounting>,
}

impl CorpusAccounting {
    fn new(mode: &'static str) -> Self {
        Self { mode, ..Self::default() }
    }

    fn add_mode(&mut self, mode: ModeAccounting) {
        self.discovered_files += mode.discovered_files;
        self.runnable_files += mode.runnable_files;
        self.skipped_files += mode.skipped_files;
        self.expected_passes += mode.expected_passes;
        self.expected_failures += mode.expected_failures;
        self.ignored_exit_files += mode.ignored_exit_files;
        self.unavailable_prerequisites.extend(mode.unavailable_prerequisites.iter().cloned());
        self.modes.push(mode);
    }

    fn to_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n  \"mode\": ");
        push_json_string(&mut out, self.mode);
        write!(out, ",\n  \"discovered_files\": {}", self.discovered_files).unwrap();
        write!(out, ",\n  \"runnable_files\": {}", self.runnable_files).unwrap();
        write!(out, ",\n  \"skipped_files\": {}", self.skipped_files).unwrap();
        write!(out, ",\n  \"expected_passes\": {}", self.expected_passes).unwrap();
        write!(out, ",\n  \"expected_failures\": {}", self.expected_failures).unwrap();
        write!(out, ",\n  \"ignored_exit_files\": {}", self.ignored_exit_files).unwrap();
        out.push_str(",\n  \"unavailable_prerequisites\": ");
        push_unavailable_json(&mut out, &self.unavailable_prerequisites, 2);
        out.push_str(",\n  \"modes\": [");
        for (idx, mode) in self.modes.iter().enumerate() {
            if idx != 0 {
                out.push(',');
            }
            mode.push_json(&mut out);
        }
        out.push_str("\n  ]\n}\n");
        out
    }
}

struct ModeAccounting {
    mode: &'static str,
    root: String,
    discovered_files: usize,
    runnable_files: usize,
    skipped_files: usize,
    expected_passes: usize,
    expected_failures: usize,
    ignored_exit_files: usize,
    skipped_reasons: BTreeMap<&'static str, usize>,
    unavailable_prerequisites: Vec<UnavailablePrerequisite>,
}

impl ModeAccounting {
    fn new(mode: Mode, root: &Path) -> Self {
        Self {
            mode: mode.to_str(),
            root: root.display().to_string(),
            discovered_files: 0,
            runnable_files: 0,
            skipped_files: 0,
            expected_passes: 0,
            expected_failures: 0,
            ignored_exit_files: 0,
            skipped_reasons: BTreeMap::new(),
            unavailable_prerequisites: Vec::new(),
        }
    }

    fn push_json(&self, out: &mut String) {
        out.push_str("\n    {\n      \"mode\": ");
        push_json_string(out, self.mode);
        out.push_str(",\n      \"root\": ");
        push_json_string(out, &self.root);
        write!(out, ",\n      \"discovered_files\": {}", self.discovered_files).unwrap();
        write!(out, ",\n      \"runnable_files\": {}", self.runnable_files).unwrap();
        write!(out, ",\n      \"skipped_files\": {}", self.skipped_files).unwrap();
        write!(out, ",\n      \"expected_passes\": {}", self.expected_passes).unwrap();
        write!(out, ",\n      \"expected_failures\": {}", self.expected_failures).unwrap();
        write!(out, ",\n      \"ignored_exit_files\": {}", self.ignored_exit_files).unwrap();
        out.push_str(",\n      \"skipped_reasons\": {");
        for (idx, (reason, count)) in self.skipped_reasons.iter().enumerate() {
            if idx != 0 {
                out.push(',');
            }
            out.push_str("\n        ");
            push_json_string(out, reason);
            write!(out, ": {count}").unwrap();
        }
        if !self.skipped_reasons.is_empty() {
            out.push_str("\n      ");
        }
        out.push('}');
        out.push_str(",\n      \"unavailable_prerequisites\": ");
        push_unavailable_json(out, &self.unavailable_prerequisites, 6);
        out.push_str("\n    }");
    }
}

#[derive(Clone)]
struct UnavailablePrerequisite {
    path: String,
    reason: String,
}

fn push_unavailable_json(out: &mut String, unavailable: &[UnavailablePrerequisite], indent: usize) {
    out.push('[');
    for (idx, item) in unavailable.iter().enumerate() {
        if idx != 0 {
            out.push(',');
        }
        write!(out, "\n{}{{\n{}  \"path\": ", " ".repeat(indent + 2), " ".repeat(indent + 2)).unwrap();
        push_json_string(out, &item.path);
        write!(out, ",\n{}  \"reason\": ", " ".repeat(indent + 2)).unwrap();
        push_json_string(out, &item.reason);
        write!(out, "\n{} }}", " ".repeat(indent + 2)).unwrap();
    }
    if !unavailable.is_empty() {
        write!(out, "\n{}", " ".repeat(indent)).unwrap();
    }
    out.push(']');
}

fn push_json_string(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => write!(out, "\\u{:04x}", ch as u32).unwrap(),
            ch => out.push(ch),
        }
    }
    out.push('"');
}

#[derive(Clone, Copy)]
enum Mode {
    Ui,
    SolcSolidity,
    SolcYul,
}

impl Mode {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "ui" => Self::Ui,
            "solc-solidity" => Self::SolcSolidity,
            "solc-yul" => Self::SolcYul,
            _ => return None,
        })
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Ui => "ui",
            Self::SolcSolidity => "solc-solidity",
            Self::SolcYul => "solc-yul",
        }
    }

    fn corpus_path(self) -> &'static str {
        match self {
            Self::Ui => "tests/ui/",
            Self::SolcSolidity => "testdata/solidity/test/",
            Self::SolcYul => "testdata/solidity/test/libyul/",
        }
    }

    fn is_solc(self) -> bool {
        matches!(self, Self::SolcSolidity | Self::SolcYul)
    }

    fn allows_yul(self) -> bool {
        !matches!(self, Self::SolcSolidity)
    }

    fn is_candidate_file(self, path: &Path) -> bool {
        path.extension().is_some_and(|ext| ext == "sol" || (self.allows_yul() && ext == "yul"))
    }

    fn skip_reason(self, path: &Path) -> Option<&'static str> {
        match self {
            Self::Ui => None,
            Self::SolcSolidity => solc::solidity::should_skip(path).err(),
            Self::SolcYul => solc::yul::should_skip(path).err(),
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Clone, Copy)]
struct MyConfig<'a> {
    mode: Mode,
    tmp_dir: &'a Path,
}
