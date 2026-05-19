//! Solar test runner.
//!
//! This crate is invoked in `crates/solar/tests.rs` with the path to the `solar` binary.

#![allow(unreachable_pub)]

use eyre::{Context, Result, eyre};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};
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

    let ledger_path = std::env::var_os("SOLAR_BASELINE_LEDGER").map(|path| {
        let path = PathBuf::from(path);
        if path.is_absolute() { path } else { workspace_root().join(path) }
    });
    let started_at = Instant::now();
    let mut ledger = ledger_path.as_ref().map(|_| BaselineLedger::new());

    let tmp_dir = tempfile::tempdir()?;
    let tmp_dir = &*Box::leak(tmp_dir.path().to_path_buf().into_boxed_path());
    let mut result = Ok(());
    for &mode in modes {
        let cfg = MyConfig::<'static> { mode, tmp_dir };
        let config = config(cmd, &args, mode);

        let mode_started_at = Instant::now();
        let counts = ledger.as_ref().map(|_| count_corpus(&config, cfg));

        let text_emitter: Box<dyn ui_test::status_emitter::StatusEmitter> = args.format.into();
        let gha_emitter = ui_test::status_emitter::Gha { name: mode.to_string(), group: true };
        let status_emitter = (text_emitter, gha_emitter);

        let mode_result = ui_test::run_tests_generic(
            vec![config],
            move |path, config| file_filter(path, config, cfg),
            move |config, contents| per_file_config(config, contents, cfg),
            status_emitter,
        );

        if let Some(ledger) = &mut ledger {
            let mut counts = counts.expect("ledger counts should exist when ledger is enabled");
            if mode_result.is_err() {
                counts.failed = 1;
                counts.passed = counts.passed.saturating_sub(1);
            }
            ledger.oracles.push(BaselineOracle {
                name: mode.to_string(),
                corpus_root: mode.corpus_root().into(),
                counts,
                elapsed_ms: mode_started_at.elapsed().as_millis(),
            });
        }

        if mode_result.is_err() {
            result = mode_result;
            break;
        }
    }

    if let (Some(path), Some(mut ledger)) = (ledger_path, ledger) {
        ledger.elapsed_ms = started_at.elapsed().as_millis();
        ledger
            .write(&path)
            .wrap_err_with(|| format!("failed to write baseline ledger to {}", path.display()))?;
    }

    result
}

fn config(cmd: &'static Path, args: &ui_test::Args, mode: Mode) -> ui_test::Config {
    let root = workspace_root();

    let path = mode.corpus_root();
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

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap()
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

fn count_corpus(config: &ui_test::Config, cfg: MyConfig<'_>) -> BaselineCounts {
    let mut counts = BaselineCounts::default();
    count_corpus_dir(&config.root_dir, config, cfg, &mut counts);
    counts.passed = counts.total.saturating_sub(counts.skipped);
    counts
}

fn count_corpus_dir(
    dir: &Path,
    config: &ui_test::Config,
    cfg: MyConfig<'_>,
    counts: &mut BaselineCounts,
) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            count_corpus_dir(&path, config, cfg, counts);
        } else if let Some(include) = file_filter(&path, config, cfg) {
            counts.total += 1;
            if include {
                counts.runnable += 1;
            } else {
                counts.skipped += 1;
            }
        }
    }
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
    let expected_errors = errors::Error::load_solc(src);
    let expected_error = expected_errors.iter().find(|e| e.is_error());
    let code = if let Some(expected_error) = expected_error {
        // Expect failure only for parser errors, otherwise ignore exit code.
        if expected_error.solc_kind.is_some_and(|kind| kind.is_parser_error()) {
            Some(1)
        } else {
            None
        }
    } else {
        Some(0)
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

    fn corpus_root(self) -> &'static str {
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

struct BaselineLedger {
    elapsed_ms: u128,
    oracles: Vec<BaselineOracle>,
}

impl BaselineLedger {
    fn new() -> Self {
        Self { elapsed_ms: 0, oracles: Vec::new() }
    }

    fn write(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        writeln!(file, "{{")?;
        writeln!(file, "  \"schema\": \"solar-baseline-ledger-v1\",")?;
        writeln!(file, "  \"tool\": {{")?;
        writeln!(file, "    \"name\": \"solar\",")?;
        writeln!(file, "    \"version\": \"{}\"", json_escape(env!("CARGO_PKG_VERSION")))?;
        writeln!(file, "  }},")?;
        writeln!(file, "  \"elapsed_ms\": {},", self.elapsed_ms)?;
        writeln!(file, "  \"oracles\": [")?;
        for (idx, oracle) in self.oracles.iter().enumerate() {
            oracle.write(&mut file, idx + 1 == self.oracles.len())?;
        }
        writeln!(file, "  ]")?;
        writeln!(file, "}}")?;
        Ok(())
    }
}

struct BaselineOracle {
    name: String,
    corpus_root: String,
    counts: BaselineCounts,
    elapsed_ms: u128,
}

impl BaselineOracle {
    fn write(&self, file: &mut fs::File, last: bool) -> Result<()> {
        writeln!(file, "    {{")?;
        writeln!(file, "      \"name\": \"{}\",", json_escape(&self.name))?;
        writeln!(file, "      \"corpus_root\": \"{}\",", json_escape(&self.corpus_root))?;
        writeln!(file, "      \"counts\": {{")?;
        writeln!(file, "        \"corpus\": {},", self.counts.total)?;
        writeln!(file, "        \"runnable\": {},", self.counts.runnable)?;
        writeln!(file, "        \"passed\": {},", self.counts.passed)?;
        writeln!(file, "        \"failed\": {},", self.counts.failed)?;
        writeln!(file, "        \"skipped\": {}", self.counts.skipped)?;
        writeln!(file, "      }},")?;
        writeln!(file, "      \"elapsed_ms\": {}", self.elapsed_ms)?;
        writeln!(file, "    }}{}", if last { "" } else { "," })?;
        Ok(())
    }
}

#[derive(Default)]
struct BaselineCounts {
    total: u64,
    runnable: u64,
    passed: u64,
    failed: u64,
    skipped: u64,
}

fn json_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => escaped.push_str(&format!("\\u{:04x}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}
