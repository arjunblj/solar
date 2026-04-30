use crate::utils::path_contains_curry;
use std::{collections::BTreeMap, path::Path};

const SKIP_REASONS: &[(&str, &str)] = &[
    ("blobbasefee_identifier_pre_cancun", "EVM version-aware parsing"),
    ("blobhash_pre_cancun", "EVM version-aware parsing"),
    ("clash_with_non_reserved_pure_yul_builtin", "EVM version-aware parsing"),
    ("clash_with_reserved_pure_yul_builtin", "EVM version-aware parsing"),
    ("clash_with_reserved_pure_yul_builtin_eof", "EVM version-aware parsing"),
    ("clz", "EVM version-aware parsing"),
    ("datacopy_shadowing", "Yul object syntax"),
    ("dataoffset_shadowing", "Yul object syntax"),
    ("datasize_shadowing", "Yul object syntax"),
    ("eof_names_reserved_in_eof", "EVM version-aware parsing"),
    ("extcall_function_in_eof", "EVM version-aware parsing"),
    ("extdelegatecall_function_in_eof", "EVM version-aware parsing"),
    ("extstaticcall_function_in_eof", "EVM version-aware parsing"),
    ("for_statement_nested_continue", "post-parse validation"),
    ("linkersymbol_invalid_redefine_builtin", "post-parse validation"),
    ("linkersymbol_shadowing", "Yul object syntax"),
    ("loadimmutable_shadowing", "Yul object syntax"),
    ("mcopy_as_identifier_pre_cancun", "EVM version-aware parsing"),
    ("mcopy_pre_cancun", "EVM version-aware parsing"),
    ("number_literal_2", "post-parse validation"),
    ("number_literal_3", "post-parse validation"),
    ("number_literal_4", "post-parse validation"),
    ("number_literals_2", "post-parse validation"),
    ("number_literals_3", "post-parse validation"),
    ("number_literals_4", "post-parse validation"),
    ("pc_disallowed", "post-parse validation"),
    ("setimmutable_shadowing", "Yul object syntax"),
    ("tstore_tload_as_identifiers_pre_cancun", "EVM version-aware parsing"),
    ("unicode_comment_direction_override", "unicode comment validation"),
];

pub(crate) fn skip_summary() -> BTreeMap<&'static str, usize> {
    let mut summary = BTreeMap::new();
    for &(_, reason) in SKIP_REASONS {
        *summary.entry(reason).or_default() += 1;
    }

    summary.insert("not a Yul file", 0);
    summary.insert("not actually valid identifiers", 3);
    summary.insert("not implemented in the parser", 2);
    summary.insert("recursion stack overflow", 1);
    summary.insert("verbatim Yul builtin is not implemented", 1);
    summary
}

pub(crate) fn should_skip(path: &Path) -> Result<(), &'static str> {
    let path_contains = path_contains_curry(path);

    if path_contains("/recursion_depth.yul") {
        return Err("recursion stack overflow");
    }

    if path_contains("/verbatim") {
        return Err("verbatim Yul builtin is not implemented");
    }

    if path_contains("/period_in_identifier")
        || path_contains("/dot_middle")
        || path_contains("/leading_and_trailing_dots")
    {
        // Why does Solc parse periods as part of Yul identifiers?
        // `yul-identifier` is the same as `solidity-identifier`, which disallows periods:
        // https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.YulIdentifier
        return Err("not actually valid identifiers");
    }

    if path_contains("objects/conflict_") || path_contains("objects/code.yul") {
        // Not the parser's job to check conflicting names.
        return Err("not implemented in the parser");
    }

    if path_contains(".sol") {
        return Err("not a Yul file");
    }

    let stem = path.file_stem().unwrap().to_str().unwrap();
    if let Some(&(_, reason)) = SKIP_REASONS.iter().find(|&&(skipped, _)| skipped == stem) {
        return Err(reason);
    };

    Ok(())
}
