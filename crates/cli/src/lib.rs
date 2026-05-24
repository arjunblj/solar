#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/solar/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/paradigmxyz/solar/main/assets/favicon.ico"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use clap::Parser as _;
use solar_interface::{Result, Session, config::CompilerOutput};
use solar_sema::CompilerRef;
use std::{
    collections::{BTreeMap, BTreeSet},
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
            serde_json::Map::new(),
            vec![standard_json_error("IOError", format!("failed to read stdin: {error}"))],
        );
        return Ok(());
    }

    let input = match serde_json::from_str::<serde_json::Value>(&input) {
        Ok(input) => input,
        Err(error) => {
            write_standard_json_output(
                opts.pretty_json,
                serde_json::Map::new(),
                vec![standard_json_error("JSONError", format!("invalid JSON input: {error}"))],
            );
            return Ok(());
        }
    };

    let mut errors = validate_standard_json_input(&input);
    let selections = standard_json_selected_outputs(&input, &mut errors);

    let mut contracts = serde_json::Map::new();
    if errors.is_empty() && selections.iter().any(|selection| selection.is_compilation_output()) {
        let output = compile_standard_json_outputs(opts.clone(), &input, &selections)?;
        errors.extend(output.errors);
        contracts = output.contracts;
    }

    write_standard_json_output(opts.pretty_json, contracts, errors);
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum StandardJsonOutputSelection {
    Abi,
    MethodIdentifiers,
}

impl StandardJsonOutputSelection {
    fn is_compilation_output(self) -> bool {
        matches!(self, Self::Abi | Self::MethodIdentifiers)
    }
}

#[derive(Default)]
struct StandardJsonCompileOutput {
    contracts: serde_json::Map<String, serde_json::Value>,
    errors: Vec<serde_json::Value>,
}

fn standard_json_selected_outputs(
    input: &serde_json::Value,
    errors: &mut Vec<serde_json::Value>,
) -> BTreeSet<StandardJsonOutputSelection> {
    let mut selections = BTreeSet::new();
    let Some(settings) = input.get("settings") else { return selections };
    let Some(settings) = settings.as_object() else { return selections };
    let Some(selected_outputs) = settings.get("outputSelection") else { return selections };

    let Some(selected_outputs) = selected_outputs.as_object() else {
        errors.push(standard_json_error("JSONError", "settings.outputSelection must be an object"));
        return selections;
    };

    for (source_name, contracts) in selected_outputs {
        let Some(contracts) = contracts.as_object() else {
            errors.push(standard_json_error(
                "JSONError",
                format!("settings.outputSelection.{source_name} must be an object"),
            ));
            continue;
        };

        for (contract_name, outputs) in contracts {
            let Some(outputs) = outputs.as_array() else {
                errors.push(standard_json_error(
                    "JSONError",
                    format!(
                        "settings.outputSelection.{source_name}.{contract_name} must be an array"
                    ),
                ));
                continue;
            };

            for output in outputs {
                let Some(output) = output.as_str() else {
                    errors.push(standard_json_error(
                        "JSONError",
                        format!(
                            "settings.outputSelection.{source_name}.{contract_name} entries must be strings"
                        ),
                    ));
                    continue;
                };

                match output {
                    "abi" => {
                        selections.insert(StandardJsonOutputSelection::Abi);
                    }
                    "evm.methodIdentifiers" => {
                        selections.insert(StandardJsonOutputSelection::MethodIdentifiers);
                    }
                    "*" => {
                        selections.insert(StandardJsonOutputSelection::Abi);
                        selections.insert(StandardJsonOutputSelection::MethodIdentifiers);
                        errors.push(standard_json_error(
                            "JSONError",
                            "unsupported selected output: * includes codegen/runtime artifacts that Solar does not support via standard JSON yet",
                        ));
                    }
                    unsupported => errors.push(standard_json_error(
                        "JSONError",
                        format!("unsupported selected output: {unsupported}"),
                    )),
                }
            }
        }
    }

    selections
}

fn compile_standard_json_outputs(
    mut opts: Opts,
    input: &serde_json::Value,
    selections: &BTreeSet<StandardJsonOutputSelection>,
) -> Result<StandardJsonCompileOutput> {
    let mut output = StandardJsonCompileOutput::default();

    opts.emit.clear();
    if selections.contains(&StandardJsonOutputSelection::Abi) {
        opts.emit.push(CompilerOutput::Abi);
    }
    if selections.contains(&StandardJsonOutputSelection::MethodIdentifiers) {
        opts.emit.push(CompilerOutput::Hashes);
    }

    run_compiler_with(opts, |compiler| {
        let sess = compiler.gcx().sess;
        let mut pcx = compiler.parse();
        let mut paths = Vec::new();
        if let Some(sources) = input.get("sources").and_then(serde_json::Value::as_object) {
            for (name, source) in sources {
                if let Some(content) = source.get("content").and_then(serde_json::Value::as_str) {
                    sess.source_map()
                        .new_source_file(std::path::PathBuf::from(name), content.to_string())
                        .map_err(|error| {
                            sess.dcx
                                .err(format!(
                                    "failed to load standard JSON source {name:?}: {error}"
                                ))
                                .emit()
                        })?;
                    paths.push(name.as_str());
                }
            }
        }
        pcx.par_load_files(paths)?;
        pcx.parse();

        let ControlFlow::Continue(()) = compiler.lower_asts()? else { return Ok(()) };
        compiler.drop_asts();
        let ControlFlow::Continue(()) = compiler.analysis()? else { return Ok(()) };

        let gcx = compiler.gcx();
        for id in gcx.hir.contract_ids() {
            let fqn = gcx.contract_fully_qualified_name(id).to_string();
            let Some((source, contract)) = fqn.rsplit_once(':') else { continue };
            let source_entry = output
                .contracts
                .entry(source.to_string())
                .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
            let serde_json::Value::Object(source_contracts) = source_entry else { continue };
            let mut contract_output = serde_json::Map::new();

            if selections.contains(&StandardJsonOutputSelection::Abi) {
                contract_output
                    .insert("abi".to_string(), serde_json::to_value(gcx.contract_abi(id)).unwrap());
            }
            if selections.contains(&StandardJsonOutputSelection::MethodIdentifiers) {
                let mut method_identifiers = BTreeMap::new();
                for f in gcx.interface_functions(id) {
                    method_identifiers.insert(
                        gcx.item_signature(f.id.into()).to_string(),
                        alloy_primitives::hex::encode(f.selector),
                    );
                }
                contract_output.insert(
                    "evm".to_string(),
                    serde_json::json!({ "methodIdentifiers": method_identifiers }),
                );
            }

            source_contracts
                .insert(contract.to_string(), serde_json::Value::Object(contract_output));
        }

        Ok(())
    })?;

    Ok(output)
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
                for setting in settings.keys() {
                    match setting.as_str() {
                        "outputSelection" => {}
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

fn write_standard_json_output(
    pretty: bool,
    contracts: serde_json::Map<String, serde_json::Value>,
    errors: Vec<serde_json::Value>,
) {
    let mut output = serde_json::Map::new();
    output.insert("errors".to_string(), serde_json::Value::Array(errors));
    if !contracts.is_empty() {
        output.insert("contracts".to_string(), serde_json::Value::Object(contracts));
    }
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let result = if pretty {
        serde_json::to_writer_pretty(&mut stdout, &serde_json::Value::Object(output))
    } else {
        serde_json::to_writer(&mut stdout, &serde_json::Value::Object(output))
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
