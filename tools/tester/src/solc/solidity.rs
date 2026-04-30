use crate::utils::path_contains_curry;
use std::{
    ffi::OsString,
    fs,
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};

pub(crate) fn should_skip(path: &Path) -> Result<(), &'static str> {
    match classify_case(path) {
        CorpusCase::Execute => Ok(()),
        CorpusCase::Skip(reason) | CorpusCase::Unsupported(reason) => Err(reason),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CorpusCase {
    Execute,
    Skip(&'static str),
    Unsupported(&'static str),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CorpusCounters {
    pub(crate) executed: usize,
    pub(crate) skipped: usize,
    pub(crate) unsupported: usize,
}

impl CorpusCounters {
    fn record(&mut self, case: CorpusCase) {
        match case {
            CorpusCase::Execute => self.executed += 1,
            CorpusCase::Skip(_) => self.skipped += 1,
            CorpusCase::Unsupported(_) => self.unsupported += 1,
        }
    }

    pub(crate) fn from_paths<'a>(paths: impl IntoIterator<Item = &'a Path>) -> Self {
        let mut counters = Self::default();
        for path in paths {
            counters.record(classify_case(path));
        }
        counters
    }
}

pub(crate) fn classify_case(path: &Path) -> CorpusCase {
    let path_contains = path_contains_curry(path);

    if path_contains("/libyul/") {
        return CorpusCase::Skip("actually a Yul test");
    }

    if path_contains("/cmdlineTests/") {
        return CorpusCase::Unsupported("CLI tests do not have the same format as everything else");
    }

    if path_contains("/lsp/") {
        return CorpusCase::Unsupported("LSP tests do not have the same format as everything else");
    }

    if path_contains("/ASTJSON/") {
        return CorpusCase::Unsupported("no JSON AST");
    }

    if path_contains("/functionDependencyGraphTests/") || path_contains("/experimental") {
        return CorpusCase::Unsupported("solidity experimental is not implemented");
    }

    // We don't parse licenses.
    if path_contains("/license/") {
        return CorpusCase::Skip("licenses are not checked");
    }

    if path_contains("natspec") {
        return CorpusCase::Skip("natspec is not checked");
    }

    if path_contains("_direction_override") {
        return CorpusCase::Unsupported("Unicode direction override checks not implemented");
    }

    if path_contains("wrong_compiler_") {
        return CorpusCase::Skip("Solidity pragma version is not checked");
    }

    // Directories starting with `_` are not tests.
    if path_contains("/_")
        && !path.components().next_back().unwrap().as_os_str().to_str().unwrap().starts_with('_')
    {
        return CorpusCase::Skip("supporting file");
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
        return CorpusCase::Skip("manually skipped");
    };

    CorpusCase::Execute
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

#[cfg(test)]
mod tests {
    use super::{CorpusCounters, classify_case};
    use std::path::Path;

    #[test]
    fn corpus_counters_distinguish_executed_skipped_and_unsupported() {
        let paths = [
            Path::new("testdata/solidity/test/libsolidity/syntaxTests/valid.sol"),
            Path::new("testdata/solidity/test/libsolidity/syntaxTests/license/mit.sol"),
            Path::new("testdata/solidity/test/cmdlineTests/no_input/no_input.sol"),
            Path::new("testdata/solidity/test/libsolidity/syntaxTests/rational_number_exp_limit_fine.sol"),
        ];

        let counters = CorpusCounters::from_paths(paths);

        assert_eq!(counters.executed, 1);
        assert_eq!(counters.skipped, 2);
        assert_eq!(counters.unsupported, 1);
        assert!(classify_case(paths[0]).is_executed());
    }
}
