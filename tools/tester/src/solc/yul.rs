use crate::utils::path_contains_curry;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SkipCategory {
    UnsupportedSolarBehavior,
    NonParserResponsibility,
    SolcAcceptedInvalidSyntax,
    NonYulInput,
    ManualInvestigation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SkipReason {
    pub(crate) category: SkipCategory,
    pub(crate) reason: &'static str,
}

impl SkipReason {
    const fn new(category: SkipCategory, reason: &'static str) -> Self {
        Self { category, reason }
    }
}

pub(crate) fn should_skip(path: &Path) -> Result<(), &'static str> {
    should_skip_reason(path).map_err(|reason| reason.reason)
}

pub(crate) fn should_skip_reason(path: &Path) -> Result<(), SkipReason> {
    let path_contains = path_contains_curry(path);

    if path_contains("/recursion_depth.yul") {
        return Err(SkipReason::new(SkipCategory::UnsupportedSolarBehavior, "recursion stack overflow"));
    }

    if path_contains("/verbatim") {
        return Err(SkipReason::new(
            SkipCategory::UnsupportedSolarBehavior,
            "verbatim Yul builtin is not implemented",
        ));
    }

    if path_contains("/period_in_identifier")
        || path_contains("/dot_middle")
        || path_contains("/leading_and_trailing_dots")
    {
        // Why does Solc parse periods as part of Yul identifiers?
        // `yul-identifier` is the same as `solidity-identifier`, which disallows periods:
        // https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.YulIdentifier
        return Err(SkipReason::new(
            SkipCategory::SolcAcceptedInvalidSyntax,
            "not actually valid identifiers",
        ));
    }

    if path_contains("objects/conflict_") || path_contains("objects/code.yul") {
        // Not the parser's job to check conflicting names.
        return Err(SkipReason::new(
            SkipCategory::NonParserResponsibility,
            "not implemented in the parser",
        ));
    }

    if path_contains(".sol") {
        return Err(SkipReason::new(SkipCategory::NonYulInput, "not a Yul file"));
    }

    let stem = path.file_stem().unwrap().to_str().unwrap();
    #[rustfmt::skip]
    if matches!(
        stem,
        // TODO: Why should this fail?
        | "unicode_comment_direction_override"
        // TODO: Implement after parsing.
        | "number_literals_2"
        | "number_literals_3"
        | "number_literals_4"
        | "number_literal_2"
        | "number_literal_3"
        | "number_literal_4"
        | "pc_disallowed"
        | "for_statement_nested_continue"
        | "linkersymbol_invalid_redefine_builtin"
        // TODO: Implemented with Yul object syntax.
        | "datacopy_shadowing"
        | "dataoffset_shadowing"
        | "datasize_shadowing"
        | "linkersymbol_shadowing"
        | "loadimmutable_shadowing"
        | "setimmutable_shadowing"
        // TODO: EVM version-aware parsing.
        | "blobbasefee_identifier_pre_cancun"
        | "blobhash_pre_cancun"
        | "mcopy_as_identifier_pre_cancun"
        | "mcopy_pre_cancun"
        | "tstore_tload_as_identifiers_pre_cancun"
        | "eof_names_reserved_in_eof"
        | "extcall_function_in_eof"
        | "extdelegatecall_function_in_eof"
        | "extstaticcall_function_in_eof"
        | "clash_with_non_reserved_pure_yul_builtin"
        | "clash_with_reserved_pure_yul_builtin_eof"
        | "clash_with_reserved_pure_yul_builtin"
        | "clz"
    ) {
        return Err(SkipReason::new(SkipCategory::ManualInvestigation, "manually skipped"));
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{should_skip, should_skip_reason, SkipCategory, SkipReason};
    use std::path::Path;

    fn skip_reason(path: &str) -> SkipReason {
        should_skip_reason(Path::new(path)).unwrap_err()
    }

    #[test]
    fn yul_skip_accounting_distinguishes_coverage_buckets() {
        assert_eq!(
            skip_reason("test/libyul/yulOptimizerTests/recursion_depth.yul"),
            SkipReason::new(SkipCategory::UnsupportedSolarBehavior, "recursion stack overflow")
        );
        assert_eq!(
            skip_reason("test/libyul/yulParserTests/verbatim.yul"),
            SkipReason::new(
                SkipCategory::UnsupportedSolarBehavior,
                "verbatim Yul builtin is not implemented"
            )
        );
        assert_eq!(
            skip_reason("test/libyul/yulParserTests/period_in_identifier/input.yul"),
            SkipReason::new(
                SkipCategory::SolcAcceptedInvalidSyntax,
                "not actually valid identifiers"
            )
        );
        assert_eq!(
            skip_reason("test/libyul/yulParserTests/objects/conflict_code.yul"),
            SkipReason::new(SkipCategory::NonParserResponsibility, "not implemented in the parser")
        );
        assert_eq!(
            skip_reason("test/libyul/SolidityFile.sol"),
            SkipReason::new(SkipCategory::NonYulInput, "not a Yul file")
        );
        assert_eq!(
            skip_reason("test/libyul/yulParserTests/number_literals_2.yul"),
            SkipReason::new(SkipCategory::ManualInvestigation, "manually skipped")
        );
    }

    #[test]
    fn legacy_skip_api_still_returns_reason_strings() {
        assert_eq!(
            should_skip(Path::new("test/libyul/yulParserTests/number_literals_2.yul")),
            Err("manually skipped")
        );
        assert_eq!(should_skip(Path::new("test/libyul/yulParserTests/simple.yul")), Ok(()));
    }
}
