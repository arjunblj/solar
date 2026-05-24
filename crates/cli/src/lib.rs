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
            &standard_json_output(
                vec![standard_json_error("IOError", format!("failed to read stdin: {error}"))],
                BTreeMap::new(),
            ),
        );
        return Ok(());
    }

    let input = match serde_json::from_str::<serde_json::Value>(&input) {
        Ok(input) => input,
        Err(error) => {
            write_standard_json_output(
                opts.pretty_json,
                &standard_json_output(
                    vec![standard_json_error("JSONError", format!("invalid JSON input: {error}"))],
                    BTreeMap::new(),
                ),
            );
            return Ok(());
        }
    };

    let (errors, output_selection) = validate_standard_json_input(&input);
    let output = standard_json_output(
        errors,
        output_selection
            .map(|selection| selected_standard_json_contracts(&input, &selection))
            .unwrap_or_default(),
    );
    write_standard_json_output(opts.pretty_json, &output);
    Ok(())
}

#[derive(Default)]
struct StandardJsonOutputSelection {
    abi: bool,
    method_identifiers: bool,
}

#[derive(Default)]
struct StandardJsonContractOutput {
    abi: Option<Vec<serde_json::Value>>,
    evm: Option<StandardJsonEvmOutput>,
}

#[derive(Default)]
struct StandardJsonEvmOutput {
    method_identifiers: BTreeMap<String, String>,
}

fn validate_standard_json_input(
    input: &serde_json::Value,
) -> (Vec<serde_json::Value>, Option<StandardJsonOutputSelection>) {
    let mut errors = Vec::new();
    let mut output_selection = None;

    let Some(input) = input.as_object() else {
        errors.push(standard_json_error("JSONError", "standard JSON input must be an object"));
        return (errors, output_selection);
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
                    if setting == "outputSelection" {
                        output_selection = validate_output_selection(value, &mut errors);
                    } else {
                        errors.push(standard_json_error(
                            "JSONError",
                            format!("unsupported settings field: settings.{setting}"),
                        ));
                    }
                }
            }
            None => errors.push(standard_json_error("JSONError", "settings must be an object")),
        }
    }

    (errors, output_selection)
}

fn validate_output_selection(
    value: &serde_json::Value,
    errors: &mut Vec<serde_json::Value>,
) -> Option<StandardJsonOutputSelection> {
    let Some(files) = value.as_object() else {
        errors.push(standard_json_error("JSONError", "settings.outputSelection must be an object"));
        return None;
    };

    let mut selection = StandardJsonOutputSelection::default();
    for (file, contracts) in files {
        let Some(contracts) = contracts.as_object() else {
            errors.push(standard_json_error(
                "JSONError",
                format!("settings.outputSelection[{file:?}] must be an object"),
            ));
            continue;
        };
        for (contract, outputs) in contracts {
            let Some(outputs) = outputs.as_array() else {
                errors.push(standard_json_error(
                    "JSONError",
                    format!("settings.outputSelection[{file:?}][{contract:?}] must be an array"),
                ));
                continue;
            };
            for output in outputs {
                let Some(output) = output.as_str() else {
                    errors.push(standard_json_error(
                        "JSONError",
                        "settings.outputSelection entries must be strings",
                    ));
                    continue;
                };
                match output {
                    "abi" => selection.abi = true,
                    "evm.methodIdentifiers" => selection.method_identifiers = true,
                    _ => {}
                }
            }
        }
    }

    Some(selection)
}

fn selected_standard_json_contracts(
    input: &serde_json::Value,
    selection: &StandardJsonOutputSelection,
) -> BTreeMap<String, BTreeMap<String, StandardJsonContractOutput>> {
    if !selection.abi && !selection.method_identifiers {
        return BTreeMap::new();
    }

    let mut contracts = BTreeMap::new();
    let Some(sources) = input.get("sources").and_then(serde_json::Value::as_object) else {
        return contracts;
    };
    for (source_name, source) in sources {
        let Some(content) = source.get("content").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let mut source_contracts = BTreeMap::new();
        for contract in find_contract_names(content) {
            let mut output = StandardJsonContractOutput::default();
            if selection.abi {
                output.abi = Some(Vec::new());
            }
            if selection.method_identifiers {
                output.evm = Some(StandardJsonEvmOutput { method_identifiers: BTreeMap::new() });
            }
            source_contracts.insert(contract, output);
        }
        if !source_contracts.is_empty() {
            contracts.insert(source_name.clone(), source_contracts);
        }
    }
    contracts
}

fn find_contract_names(content: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut tokens = content
        .split(|c: char| !(c == '_' || c == '$' || c.is_ascii_alphanumeric()))
        .filter(|token| !token.is_empty());
    while let Some(token) = tokens.next() {
        if matches!(token, "contract" | "interface" | "library") {
            if let Some(name) = tokens.next() {
                names.insert(name.to_string());
            }
        }
    }
    names
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

fn standard_json_output(
    errors: Vec<serde_json::Value>,
    contracts: BTreeMap<String, BTreeMap<String, StandardJsonContractOutput>>,
) -> serde_json::Value {
    let mut output = serde_json::Map::new();
    output.insert("errors".into(), serde_json::Value::Array(errors));
    if !contracts.is_empty() {
        output.insert("contracts".into(), standard_json_contracts_value(contracts));
    }
    serde_json::Value::Object(output)
}

fn standard_json_contracts_value(
    contracts: BTreeMap<String, BTreeMap<String, StandardJsonContractOutput>>,
) -> serde_json::Value {
    let mut files = serde_json::Map::new();
    for (file, contracts) in contracts {
        let mut file_contracts = serde_json::Map::new();
        for (name, contract) in contracts {
            let mut contract_output = serde_json::Map::new();
            if let Some(abi) = contract.abi {
                contract_output.insert("abi".into(), serde_json::Value::Array(abi));
            }
            if let Some(evm) = contract.evm {
                let method_identifiers = evm
                    .method_identifiers
                    .into_iter()
                    .map(|(signature, selector)| (signature, serde_json::Value::String(selector)))
                    .collect();
                let mut evm_output = serde_json::Map::new();
                evm_output.insert(
                    "methodIdentifiers".into(),
                    serde_json::Value::Object(method_identifiers),
                );
                contract_output.insert("evm".into(), serde_json::Value::Object(evm_output));
            }
            file_contracts.insert(name, serde_json::Value::Object(contract_output));
        }
        files.insert(file, serde_json::Value::Object(file_contracts));
    }
    serde_json::Value::Object(files)
}

fn write_standard_json_output(pretty: bool, output: &serde_json::Value) {
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
