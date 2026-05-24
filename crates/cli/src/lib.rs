#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/solar/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/paradigmxyz/solar/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use clap::Parser as _;
use solar_interface::{Result, Session};
use solar_sema::CompilerRef;
use std::{
    io::{self, Read, Write},
    ops::ControlFlow,
};

pub use solar_config::{self as config, Opts, UnstableOpts, version};

pub mod utils;

#[cfg(all(unix, any(target_env = "gnu", target_os = "macos")))]
pub mod signal_handler;

/// Signal handler to extract a backtrace from stack overflow.
///
/// This is a no-op because this platform doesn't support our signal handler's requirements.
#[cfg(not(all(unix, any(target_env = "gnu", target_os = "macos"))))]
pub mod signal_handler {
    #[cfg(unix)]
    use libc as _;

    /// No-op function.
    pub fn install() {}
}

// `asm` feature.
use alloy_primitives as _;

use tracing as _;

pub fn parse_args<I, T>(itr: I) -> Result<Opts, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let mut opts = Opts::try_parse_from(itr)?;
    opts.finish()?;
    Ok(opts)
}

pub fn run_compiler_args(opts: Opts) -> Result {
    if opts.standard_json {
        return run_standard_json(opts);
    }

    run_compiler_with(opts, run_default)
}

fn run_standard_json(opts: Opts) -> Result {
    let mut input = String::new();
    let mut stdin = io::stdin();
    if let Err(error) = stdin.read_to_string(&mut input) {
        write_standard_json_output(
            opts.pretty_json,
            vec![standard_json_error("IOError", format!("failed to read stdin: {error}"))],
        );
        return Ok(());
    }

    let input = match serde_json::from_str::<serde_json::Value>(&input) {
        Ok(input) => input,
        Err(error) => {
            write_standard_json_output(
                opts.pretty_json,
                vec![standard_json_error("JSONError", format!("invalid JSON input: {error}"))],
            );
            return Ok(());
        }
    };

    let errors = validate_standard_json_input(&input);
    write_standard_json_output(opts.pretty_json, errors);
    Ok(())
}

fn validate_standard_json_input(input: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut errors = Vec::new();

    let Some(input) = input.as_object() else {
        errors.push(standard_json_error("JSONError", "standard JSON input must be an object"));
        return errors;
    };

    match input.get("language") {
        Some(language) if language == "Solidity" => {}
        Some(language) if language.is_string() => errors.push(standard_json_error(
            "JSONError",
            "unsupported language: only Solidity is currently supported",
        )),
        Some(_) => errors.push(standard_json_error("JSONError", "language must be a string")),
        None => errors.push(standard_json_error("JSONError", "missing required field: language")),
    }

    match input.get("sources").and_then(serde_json::Value::as_object) {
        Some(sources) => {
            for (name, source) in sources {
                let Some(source) = source.as_object() else {
                    errors.push(standard_json_error(
                        "JSONError",
                        format!("source {name:?} must be an object"),
                    ));
                    continue;
                };

                if source.contains_key("urls") {
                    errors.push(standard_json_error(
                        "JSONError",
                        format!(
                            "source {name:?} uses unsupported urls; inline content is required"
                        ),
                    ));
                }

                match source.get("content") {
                    Some(content) if content.is_string() => {}
                    Some(_) => errors.push(standard_json_error(
                        "JSONError",
                        format!("source {name:?} content must be a string"),
                    )),
                    None => errors.push(standard_json_error(
                        "JSONError",
                        format!("source {name:?} is missing inline content"),
                    )),
                }
            }
        }
        None => errors.push(standard_json_error("JSONError", "missing required field: sources")),
    }

    if let Some(settings) = input.get("settings") {
        match settings.as_object() {
            Some(settings) => {
                for (setting, value) in settings {
                    match setting.as_str() {
                        "evmVersion" | "libraries" | "metadata" | "optimizer" => {}
                        "outputSelection" => {
                            validate_standard_json_output_selection(value, &mut errors)
                        }
                        _ => errors.push(standard_json_error(
                            "JSONError",
                            format!("unsupported settings field: settings.{setting}"),
                        )),
                    }
                }
            }
            None => errors.push(standard_json_error("JSONError", "settings must be an object")),
        }
    }

    errors
}

fn validate_standard_json_output_selection(
    output_selection: &serde_json::Value,
    errors: &mut Vec<serde_json::Value>,
) {
    let Some(source_selectors) = output_selection.as_object() else {
        errors.push(standard_json_error("JSONError", "settings.outputSelection must be an object"));
        return;
    };

    for (source_selector, contract_selectors) in source_selectors {
        let Some(contract_selectors) = contract_selectors.as_object() else {
            errors.push(standard_json_error(
                "JSONError",
                format!("settings.outputSelection[{source_selector:?}] must be an object"),
            ));
            continue;
        };

        for (contract_selector, outputs) in contract_selectors {
            let Some(outputs) = outputs.as_array() else {
                errors.push(standard_json_error(
                    "JSONError",
                    format!(
                        "settings.outputSelection[{source_selector:?}][{contract_selector:?}] must be an array"
                    ),
                ));
                continue;
            };

            for output in outputs {
                let Some(output) = output.as_str() else {
                    errors.push(standard_json_error(
                        "JSONError",
                        format!(
                            "settings.outputSelection[{source_selector:?}][{contract_selector:?}] entries must be strings"
                        ),
                    ));
                    continue;
                };

                if !is_supported_standard_json_output(output) {
                    errors.push(standard_json_error(
                        "JSONError",
                        format!(
                            "unsupported outputSelection entry: settings.outputSelection[{source_selector:?}][{contract_selector:?}] contains {output:?}"
                        ),
                    ));
                }
            }
        }
    }
}

fn is_supported_standard_json_output(output: &str) -> bool {
    matches!(output, "abi" | "evm.methodIdentifiers")
}

fn standard_json_error(error_type: &'static str, message: impl Into<String>) -> serde_json::Value {
    let message = message.into();
    serde_json::json!({
        "component": "general",
        "errorCode": "0",
        "formattedMessage": message,
        "message": message,
        "severity": "error",
        "type": error_type,
    })
}

fn write_standard_json_output(pretty: bool, errors: Vec<serde_json::Value>) {
    let output = serde_json::json!({ "errors": errors });
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let result = if pretty {
        serde_json::to_writer_pretty(&mut stdout, &output)
    } else {
        serde_json::to_writer(&mut stdout, &output)
    };
    if result.is_ok() {
        let _ = writeln!(stdout);
    }
}

fn run_default(compiler: &mut CompilerRef<'_>) -> Result {
    let sess = compiler.gcx().sess;
    if sess.opts.language.is_yul() && !sess.opts.unstable.parse_yul {
        return Err(sess.dcx.err("Yul is not supported yet").emit());
    }

    let mut pcx = compiler.parse();

    // Partition arguments into three categories:
    // - `stdin`: `-`, occurrences after the first are ignored
    // - remappings: `[context:]prefix=path`, already parsed as part of `Opts`
    // - paths: everything else
    let mut seen_stdin = false;
    let mut paths = Vec::new();
    for arg in sess.opts.input.iter().map(String::as_str) {
        if arg == "-" {
            if !seen_stdin {
                pcx.load_stdin()?;
            }
            seen_stdin = true;
            continue;
        }

        if arg.contains('=') {
            continue;
        }

        paths.push(arg);
    }

    pcx.par_load_files(paths)?;

    pcx.parse();

    if compiler.gcx().sources.is_empty() {
        let msg = "no files found";
        let note = "if you wish to use the standard input, please specify `-` explicitly";
        return Err(sess.dcx.err(msg).note(note).emit());
    }

    let ControlFlow::Continue(()) = compiler.lower_asts()? else { return Ok(()) };
    compiler.drop_asts();
    let ControlFlow::Continue(()) = compiler.analysis()? else { return Ok(()) };

    Ok(())
}

fn run_compiler_with(opts: Opts, f: impl FnOnce(&mut CompilerRef<'_>) -> Result + Send) -> Result {
    let mut sess = Session::new(opts);
    sess.infer_language();
    sess.validate()?;

    let mut compiler = solar_sema::Compiler::new(sess);
    compiler.enter_mut(|compiler| {
        let mut r = f(compiler);
        r = r.and(finish_diagnostics(compiler.gcx().sess));
        r
    })
}

fn finish_diagnostics(sess: &Session) -> Result {
    sess.dcx.print_error_count()
}
