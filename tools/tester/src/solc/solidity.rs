use crate::utils::path_contains_curry;
use std::{
    ffi::OsString,
    fs,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

pub(crate) fn should_skip(path: &Path) -> Result<(), SkipReason> {
    let path_contains = path_contains_curry(path);

    if path_contains("/libyul/") {
        return Err(SkipReason::YulTest);
    }

    if path_contains("/cmdlineTests/") {
        return Err(SkipReason::CmdlineTest);
    }

    if path_contains("/lsp/") {
        return Err(SkipReason::LspTest);
    }

    if path_contains("/ASTJSON/") {
        return Err(SkipReason::JsonAst);
    }

    if path_contains("/functionDependencyGraphTests/") || path_contains("/experimental") {
        return Err(SkipReason::Experimental);
    }

    // We don't parse licenses.
    if path_contains("/license/") {
        return Err(SkipReason::License);
    }

    if path_contains("natspec") {
        return Err(SkipReason::NatSpec);
    }

    if path_contains("_direction_override") {
        return Err(SkipReason::DirectionOverride);
    }

    if path_contains("wrong_compiler_") {
        return Err(SkipReason::PragmaVersion);
    }

    // Directories starting with `_` are not tests.
    if path_contains("/_")
        && !path.components().next_back().unwrap().as_os_str().to_str().unwrap().starts_with('_')
    {
        return Err(SkipReason::SupportingFile);
    }

    let stem = path.file_stem().unwrap().to_str().unwrap();
    #[rustfmt::skip]
    if matches!(
        stem,
        // Exponent is too large, but apparently it's fine in Solc because the result is 0 or it gets evaluated at compile time.
        | "rational_number_exp_limit_fine"
        | "exponent_fine"
        | "rational_large_1"
        | "constant_initialized_with_unlimited_arithmetic_expression"
        // `address payable` is allowed by the grammar (see `elementary-type-name`), but not by Solc.
        | "address_payable_type_expression"
        | "mapping_from_address_payable"
        // `hex` is not a keyword, looks like just a Solc limitation?
        | "hex_as_identifier"
        // TODO: These should be checked after parsing.
        | "assembly_invalid_type"
        | "assembly_dialect_leading_space"
        // `1wei` gets lexed as two different tokens, I think it's fine.
        | "invalid_denomination_no_whitespace"
        // Not actually a broken version, we just don't check "^0 and ^1".
        | "broken_version_1"
        // TODO: CBA to implement.
        | "unchecked_while_body"
        // TODO: EVM version-aware parsing.
        | "basefee_berlin_function"
        | "prevrandao_allowed_function_pre_paris"
        | "blobbasefee_shanghai_function"
        | "blobhash_pre_cancun"
        | "mcopy_as_identifier_pre_cancun"
        | "tload_tstore_not_reserved_before_cancun"
        | "blobhash_pre_cancun_not_reserved"
        | "clz_reserved_osaka"
        // Arbitrary `pragma experimental` values are allowed by Solc apparently.
        | "experimental_test_warning"
        // "." is not a valid import path.
        | "boost_filesystem_bug"
        // Invalid UTF-8 is not supported.
        | "invalid_utf8_sequence"
        // Validation is in solar's AST stage (https://github.com/paradigmxyz/solar/pull/120).
        | "empty_enum"

        // Data locations are checked after parsing.
        | "stopAfterParsingError"
        | "state_variable_storage_named_transient"
        | "transient_local_variable"
        | "transient_function_type_with_transient_param"
        | "invalid_state_variable_location"
        | "location_specifiers_for_state_variables"

        // Mapping key types are checked in sema.
        | "mapping_nonelementary_key_1"
        | "mapping_nonelementary_key_4"
    ) {
        return Err(SkipReason::Manual);
    };

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SkipReason {
    CmdlineTest,
    DirectionOverride,
    Experimental,
    JsonAst,
    License,
    LspTest,
    Manual,
    NatSpec,
    PragmaVersion,
    SupportingFile,
    YulTest,
}

impl SkipReason {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::CmdlineTest => "cmdline-test",
            Self::DirectionOverride => "direction-override",
            Self::Experimental => "experimental",
            Self::JsonAst => "json-ast",
            Self::License => "license",
            Self::LspTest => "lsp-test",
            Self::Manual => "manual",
            Self::NatSpec => "natspec",
            Self::PragmaVersion => "pragma-version",
            Self::SupportingFile => "supporting-file",
            Self::YulTest => "yul-test",
        }
    }

    pub(crate) const fn description(self) -> &'static str {
        match self {
            Self::CmdlineTest => "CLI tests do not have the same format as everything else",
            Self::DirectionOverride => "Unicode direction override checks not implemented",
            Self::Experimental => "solidity experimental is not implemented",
            Self::JsonAst => "no JSON AST",
            Self::License => "licenses are not checked",
            Self::LspTest => "LSP tests do not have the same format as everything else",
            Self::Manual => "manually skipped",
            Self::NatSpec => "natspec is not checked",
            Self::PragmaVersion => "Solidity pragma version is not checked",
            Self::SupportingFile => "supporting file",
            Self::YulTest => "actually a Yul test",
        }
    }
}

impl std::fmt::Display for SkipReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label(), self.description())
    }
}

pub(crate) fn base_args() -> Vec<&'static str> {
    let mut args = vec!["-j1", "--error-format=rustc-json", "-Zui-testing", "-Zparse-yul"];
    append_canary_args(&mut args);
    args
}

fn append_canary_args(args: &mut Vec<&'static str>) {
    // Keep future corpus canaries in one obvious place. Once the Solidity corpus can run semantic
    // checks broadly, append `-Ztypeck` here instead of scattering mode-specific argv edits.
    let _ = args;
}

/// Handles `====` delimiters in a solc test file, and creates temporary files as necessary.
///
/// Returns `true` if it contains delimiters and the caller should not compile the original file.
#[must_use]
pub(crate) fn handle_delimiters(
    src: &str,
    path: &Path,
    tmp_dir: &Path,
    mut arg: impl FnMut(OsString),
) -> bool {
    if has_delimiters(src) {
        split_sources(src, path, tmp_dir, arg)
    } else {
        arg("-I".into());
        arg(path.parent().unwrap().into());
        false
    }
}

fn has_delimiters(src: &str) -> bool {
    // We currently only care about Source and ExternalSource which start a line with `==== `.
    src.contains("==== ")
}

#[must_use]
fn split_sources(src: &str, path: &Path, tmp_dir: &Path, mut arg: impl FnMut(OsString)) -> bool {
    let mut tmp_dir2 = None;
    let make_tmp_dir = || {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let path = tmp_dir.join(format!(
            "{}-{}",
            path.file_stem().unwrap().to_str().unwrap(),
            COUNTER.fetch_add(1, Ordering::Relaxed),
        ));
        std::fs::create_dir(&path).unwrap();
        path
    };
    let mut lines = src.lines().peekable();
    let mut add_import_path = false;
    while let Some(line) = lines.next() {
        if let Some(mut name) = source_delim(line) {
            if name == "////" {
                name = "test.sol";
            }

            let mut contents = String::with_capacity(src.len());
            while lines.peek().is_some_and(|l| !l.starts_with("====")) {
                contents.push_str(lines.next().unwrap());
                contents.push('\n');
            }

            let tmp_dir = tmp_dir2.get_or_insert_with(make_tmp_dir);
            let path = tmp_dir.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, contents).unwrap();
            arg(path.into());
        } else if let Some(eq) = external_source_delim(line) {
            if eq.contains('=') {
                arg(eq.into());
            }
            add_import_path = true;
        } else {
            // Sometimes `==== Source: ... ====` is missing after external sources.
            let mut contents = String::with_capacity(src.len());
            while let Some(&line) = lines.peek() {
                if line.starts_with("====") {
                    break;
                }
                lines.next();
                contents.push_str(line);
                contents.push('\n');
            }
            let tmp_dir = tmp_dir2.get_or_insert_with(make_tmp_dir);
            let path = tmp_dir.join("test.sol");
            fs::write(&path, contents).unwrap();
            arg(path.into());
        }
    }
    if let Some(tmp_dir) = &tmp_dir2 {
        arg("-I".into());
        arg(tmp_dir.into());
    }
    if add_import_path {
        arg("-I".into());
        arg(path.parent().unwrap().into());
    }
    tmp_dir2.is_some()
}

// https://github.com/argotorg/solidity/blob/ac54fe1972f25227f9932c8b224ef119360b0e2d/test/TestCaseReader.cpp#L111
fn source_delim(line: &str) -> Option<&str> {
    line.strip_prefix("==== Source:").and_then(|s| s.strip_suffix("====")).map(str::trim)
}

fn external_source_delim(line: &str) -> Option<&str> {
    line.strip_prefix("==== ExternalSource:").and_then(|s| s.strip_suffix("====")).map(str::trim)
}
