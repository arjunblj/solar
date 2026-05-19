//! Solar test runner.
//!
//! This crate is invoked in `crates/solar/tests.rs` with the path to the `solar` binary.

#![allow(unreachable_pub)]

use eyre::{Result, eyre};
use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
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

    let baseline_path = std::env::var_os("SOLAR_BASELINE_LEDGER").map(baseline_path);
    let baseline_started_at = std::time::Instant::now();
    let mut baseline = baseline_path.map(|path| BaselineLedger::new(path, cmd));

    let tmp_dir = tempfile::tempdir()?;
    let tmp_dir = &*Box::leak(tmp_dir.path().to_path_buf().into_boxed_path());
    for &mode in modes {
        let cfg = MyConfig::<'static> { mode, tmp_dir };
        let config = config(cmd, &args, mode);
        let mut baseline_mode = baseline.as_ref().map(|_| BaselineMode::new(mode, &config));

        let text_emitter: Box<dyn ui_test::status_emitter::StatusEmitter> = args.format.into();
        let gha_emitter = ui_test::status_emitter::Gha { name: mode.to_string(), group: true };
        let status_emitter = (text_emitter, gha_emitter);

        let started_at = std::time::Instant::now();
        let result = ui_test::run_tests_generic(
            vec![config],
            move |path, config| file_filter(path, config, cfg),
            move |config, contents| per_file_config(config, contents, cfg),
            status_emitter,
        );

        if let Some(mode) = &mut baseline_mode {
            mode.finish(result.is_ok(), started_at.elapsed());
        }
        if let (Some(baseline), Some(mode)) = (&mut baseline, baseline_mode) {
            baseline.modes.push(mode);
        }

        if let Err(err) = result {
            if let Some(baseline) = &baseline
                && let Err(write_err) = baseline.write(baseline_started_at.elapsed())
            {
                eprintln!("failed to write baseline ledger: {write_err}");
            }
            return Err(err);
        }
    }

    if let Some(baseline) = &baseline {
        baseline.write(baseline_started_at.elapsed())?;
    }

    Ok(())
}

fn config(cmd: &'static Path, args: &ui_test::Args, mode: Mode) -> ui_test::Config {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();

    let path = match mode {
        Mode::Ui => "tests/ui/",
        Mode::SolcSolidity | Mode::SolcSolidityTypeck => "testdata/solidity/test/",
        Mode::SolcYul => "testdata/solidity/test/libyul/",
    };
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
                if matches!(mode, Mode::SolcSolidity | Mode::SolcYul) {
                    args.push("--stop-after=parsing");
                }
                if matches!(mode, Mode::SolcSolidityTypeck) {
                    args.push("-Ztypeck");
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
        Mode::SolcSolidityTypeck => solc::solidity::should_skip_typeck(path).is_err(),
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
    let expected_errors = errors::Error::load_solc(src);
    let expected_error = expected_errors.iter().find(|e| e.is_error());
    let code = if let Some(expected_error) = expected_error {
        if matches!(cfg.mode, Mode::SolcSolidityTypeck) {
            // The typeck lane is a real exit-code oracle: any solc error must make Solar fail.
            Some(1)
        } else if expected_error.solc_kind.is_some_and(|kind| kind.is_parser_error()) {
            // The parser lane stops before typechecking, so only parser errors are observable.
            Some(1)
        } else {
            None
        }
    } else {
        Some(0)
    };
    config.comment_defaults.base().exit_status = code.map(Spanned::dummy).into();

    if matches!(cfg.mode, Mode::SolcSolidity | Mode::SolcSolidityTypeck) {
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
    SolcSolidityTypeck,
    SolcYul,
}

impl Mode {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "ui" => Self::Ui,
            "solc-solidity" => Self::SolcSolidity,
            "solc-solidity-typeck" => Self::SolcSolidityTypeck,
            "solc-yul" => Self::SolcYul,
            _ => return None,
        })
    }

    fn to_str(self) -> &'static str {
        match self {
            Self::Ui => "ui",
            Self::SolcSolidity => "solc-solidity",
            Self::SolcSolidityTypeck => "solc-solidity-typeck",
            Self::SolcYul => "solc-yul",
        }
    }

    fn is_solc(self) -> bool {
        matches!(self, Self::SolcSolidity | Self::SolcSolidityTypeck | Self::SolcYul)
    }

    fn allows_yul(self) -> bool {
        !matches!(self, Self::SolcSolidity | Self::SolcSolidityTypeck)
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
    path: PathBuf,
    process_command: Vec<String>,
    tester_mode: Option<String>,
    tools: Vec<BaselineTool>,
    modes: Vec<BaselineMode>,
}

impl BaselineLedger {
    fn new(path: PathBuf, cmd: &Path) -> Self {
        let solc = std::env::var_os("SOLC")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("solc"));

        Self {
            path,
            process_command: std::env::args_os()
                .map(|arg| arg.to_string_lossy().into_owned())
                .collect(),
            tester_mode: std::env::var("TESTER_MODE").ok(),
            tools: vec![BaselineTool::new("solar", cmd.into()), BaselineTool::new("solc", solc)],
            modes: Vec::new(),
        }
    }

    fn write(&self, elapsed: std::time::Duration) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }

        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("  \"schema\": \"solar-baseline-ledger-v1\",\n");
        json.push_str("  \"command\": ");
        write_json_strings(&mut json, &self.process_command);
        json.push_str(",\n");
        json.push_str("  \"tester_mode\": ");
        write_json_opt_string(&mut json, self.tester_mode.as_deref());
        json.push_str(",\n");
        let _ = writeln!(json, "  \"elapsed_ms\": {},", elapsed.as_millis());
        json.push_str("  \"tools\": [\n");
        for (idx, tool) in self.tools.iter().enumerate() {
            if idx != 0 {
                json.push_str(",\n");
            }
            tool.write_json(&mut json, "    ");
        }
        json.push_str("\n  ],\n");
        json.push_str("  \"modes\": [\n");
        for (idx, mode) in self.modes.iter().enumerate() {
            if idx != 0 {
                json.push_str(",\n");
            }
            mode.write_json(&mut json, "    ");
        }
        json.push_str("\n  ]\n}\n");

        std::fs::write(&self.path, json)
    }
}

struct BaselineTool {
    name: &'static str,
    command: String,
    available: bool,
    version: Option<String>,
}

impl BaselineTool {
    fn new(name: &'static str, command: PathBuf) -> Self {
        let output = std::process::Command::new(&command).arg("--version").output();
        let version = output.as_ref().ok().and_then(first_output_line);
        Self {
            name,
            command: display_path(&command),
            available: command.exists() || output.is_ok(),
            version,
        }
    }

    fn write_json(&self, json: &mut String, indent: &str) {
        json.push_str(indent);
        json.push_str("{\n");
        let field_indent = format!("{indent}  ");
        json.push_str(&field_indent);
        json.push_str("\"name\": ");
        write_json_string(json, self.name);
        json.push_str(",\n");
        json.push_str(&field_indent);
        json.push_str("\"command\": ");
        write_json_string(json, &self.command);
        json.push_str(",\n");
        let _ = writeln!(json, "{field_indent}\"available\": {},", self.available);
        json.push_str(&field_indent);
        json.push_str("\"version\": ");
        write_json_opt_string(json, self.version.as_deref());
        json.push('\n');
        json.push_str(indent);
        json.push('}');
    }
}

struct BaselineMode {
    name: &'static str,
    corpus_root: String,
    command: Vec<String>,
    counts: BaselineCounts,
    status: &'static str,
    elapsed_ms: u128,
}

impl BaselineMode {
    fn new(mode: Mode, config: &ui_test::Config) -> Self {
        let mut command = Vec::with_capacity(config.program.args.len() + 1);
        command.push(display_path(&config.program.program));
        command.extend(config.program.args.iter().map(|arg| arg.to_string_lossy().into_owned()));

        Self {
            name: mode.to_str(),
            corpus_root: display_path(&config.root_dir),
            command,
            counts: BaselineCounts::scan(&config.root_dir, mode),
            status: "not_run",
            elapsed_ms: 0,
        }
    }

    fn finish(&mut self, passed: bool, elapsed: std::time::Duration) {
        self.status = if passed { "passed" } else { "failed" };
        self.elapsed_ms = elapsed.as_millis();
        if passed && self.counts.pass.is_none() {
            self.counts.pass = Some(self.counts.runnable);
            self.counts.fail = Some(0);
        }
    }

    fn write_json(&self, json: &mut String, indent: &str) {
        json.push_str(indent);
        json.push_str("{\n");
        let field_indent = format!("{indent}  ");
        json.push_str(&field_indent);
        json.push_str("\"name\": ");
        write_json_string(json, self.name);
        json.push_str(",\n");
        json.push_str(&field_indent);
        json.push_str("\"corpus_root\": ");
        write_json_string(json, &self.corpus_root);
        json.push_str(",\n");
        json.push_str(&field_indent);
        json.push_str("\"command\": ");
        write_json_strings(json, &self.command);
        json.push_str(",\n");
        json.push_str(&field_indent);
        json.push_str("\"status\": ");
        write_json_string(json, self.status);
        json.push_str(",\n");
        let _ = writeln!(json, "{field_indent}\"elapsed_ms\": {},", self.elapsed_ms);
        json.push_str(&field_indent);
        json.push_str("\"counts\": ");
        self.counts.write_json(json);
        json.push('\n');
        json.push_str(indent);
        json.push('}');
    }
}

#[derive(Default)]
struct BaselineCounts {
    total: u64,
    runnable: u64,
    pass: Option<u64>,
    fail: Option<u64>,
    skip: u64,
    unsupported: u64,
    xfail: u64,
    unavailable: u64,
}

impl BaselineCounts {
    fn scan(root: &Path, mode: Mode) -> Self {
        let mut counts = Self::default();
        visit_baseline_files(root, mode, &mut counts);
        counts.runnable = counts
            .total
            .saturating_sub(counts.skip + counts.unsupported + counts.xfail + counts.unavailable);
        if matches!(mode, Mode::SolcSolidityTypeck) {
            counts.pass = Some(counts.runnable);
            counts.fail = Some(0);
        }
        counts
    }

    fn write_json(&self, json: &mut String) {
        json.push_str("{\"total\": ");
        let _ = write!(json, "{}", self.total);
        json.push_str(", \"pass\": ");
        write_json_opt_u64(json, self.pass);
        json.push_str(", \"fail\": ");
        write_json_opt_u64(json, self.fail);
        let _ = write!(
            json,
            ", \"skip\": {}, \"unsupported\": {}, \"xfail\": {}, \"unavailable\": {}}}",
            self.skip, self.unsupported, self.xfail, self.unavailable
        );
    }
}

fn visit_baseline_files(path: &Path, mode: Mode, counts: &mut BaselineCounts) {
    let Ok(metadata) = std::fs::metadata(path) else {
        counts.unavailable += 1;
        return;
    };
    if metadata.is_dir() {
        let Ok(entries) = std::fs::read_dir(path) else {
            counts.unavailable += 1;
            return;
        };
        for entry in entries.flatten() {
            visit_baseline_files(&entry.path(), mode, counts);
        }
        return;
    }

    let Some(ext) = path.extension() else { return };
    if ext != "sol" && !(mode.allows_yul() && ext == "yul") {
        return;
    }

    counts.total += 1;
    match baseline_file_bucket(path, mode) {
        BaselineBucket::Runnable => {}
        BaselineBucket::Skip => counts.skip += 1,
        BaselineBucket::Unsupported => counts.unsupported += 1,
        BaselineBucket::Xfail => counts.xfail += 1,
    }
}

enum BaselineBucket {
    Runnable,
    Skip,
    Unsupported,
    Xfail,
}

fn baseline_file_bucket(path: &Path, mode: Mode) -> BaselineBucket {
    match mode {
        Mode::Ui => BaselineBucket::Runnable,
        Mode::SolcSolidity => skip_result_bucket(solc::solidity::should_skip(path)),
        Mode::SolcSolidityTypeck => typeck_skip_result_bucket(solc::solidity::should_skip_typeck(path)),
        Mode::SolcYul => skip_result_bucket(solc::yul::should_skip(path)),
    }
}

fn skip_result_bucket<T, E>(result: Result<T, E>) -> BaselineBucket {
    if result.is_ok() { BaselineBucket::Runnable } else { BaselineBucket::Skip }
}

fn typeck_skip_result_bucket<T>(result: Result<T, solc::FixtureReason>) -> BaselineBucket {
    let Err(err) = result else { return BaselineBucket::Runnable };
    if err.reason.contains("xfail") || err.reason.contains("XFAIL") {
        BaselineBucket::Xfail
    } else if err.reason.contains("unsupported") || err.reason.contains("Unsupported") {
        BaselineBucket::Unsupported
    } else {
        BaselineBucket::Skip
    }
}

fn first_output_line(output: &std::process::Output) -> Option<String> {
    let bytes = if output.stdout.is_empty() { &output.stderr } else { &output.stdout };
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn baseline_path(path: std::ffi::OsString) -> PathBuf {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        return path;
    }

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for ancestor in cwd.ancestors() {
        if ancestor.join("PADS.md").is_file() && ancestor.join("Cargo.toml").is_file() {
            return ancestor.join(path);
        }
    }
    cwd.join(path)
}

fn write_json_opt_u64(json: &mut String, value: Option<u64>) {
    if let Some(value) = value {
        let _ = write!(json, "{value}");
    } else {
        json.push_str("null");
    }
}

fn write_json_opt_string(json: &mut String, value: Option<&str>) {
    if let Some(value) = value {
        write_json_string(json, value);
    } else {
        json.push_str("null");
    }
}

fn write_json_strings(json: &mut String, values: &[String]) {
    json.push('[');
    for (idx, value) in values.iter().enumerate() {
        if idx != 0 {
            json.push_str(", ");
        }
        write_json_string(json, value);
    }
    json.push(']');
}

fn write_json_string(json: &mut String, value: &str) {
    json.push('"');
    for ch in value.chars() {
        match ch {
            '"' => json.push_str("\\\""),
            '\\' => json.push_str("\\\\"),
            '\n' => json.push_str("\\n"),
            '\r' => json.push_str("\\r"),
            '\t' => json.push_str("\\t"),
            ch if ch.is_control() => {
                let _ = write!(json, "\\u{:04x}", ch as u32);
            }
            ch => json.push(ch),
        }
    }
    json.push('"');
}
