use crate::utils::path_contains_curry;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Outcome {
    Run,
    RecursionStackOverflow,
    UnsupportedBuiltin,
    InvalidSolcIdentifier,
    ParserOutOfScope,
    NotYul,
    ManualSkip,
}

impl Outcome {
    pub(crate) const ALL: [Self; 7] = [
        Self::Run,
        Self::RecursionStackOverflow,
        Self::UnsupportedBuiltin,
        Self::InvalidSolcIdentifier,
        Self::ParserOutOfScope,
        Self::NotYul,
        Self::ManualSkip,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::RecursionStackOverflow => "skip:recursion-stack-overflow",
            Self::UnsupportedBuiltin => "skip:unsupported-builtin",
            Self::InvalidSolcIdentifier => "skip:invalid-solc-identifier",
            Self::ParserOutOfScope => "skip:parser-out-of-scope",
            Self::NotYul => "skip:not-yul",
            Self::ManualSkip => "skip:manual",
        }
    }
}

pub(crate) fn outcome(path: &Path) -> Outcome {
    let path_contains = path_contains_curry(path);

    if path_contains("/recursion_depth.yul") {
        return Outcome::RecursionStackOverflow;
    }

    if path_contains("/verbatim") {
        return Outcome::UnsupportedBuiltin;
    }

    if path_contains("/period_in_identifier")
        || path_contains("/dot_middle")
        || path_contains("/leading_and_trailing_dots")
    {
        // Why does Solc parse periods as part of Yul identifiers?
        // `yul-identifier` is the same as `solidity-identifier`, which disallows periods:
        // https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityLexer.YulIdentifier
        return Outcome::InvalidSolcIdentifier;
    }

    if path_contains("objects/conflict_") || path_contains("objects/code.yul") {
        // Not the parser's job to check conflicting names.
        return Outcome::ParserOutOfScope;
    }

    if path_contains(".sol") {
        return Outcome::NotYul;
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
        return Outcome::ManualSkip;
    };

    Outcome::Run
}

pub(crate) fn should_skip(path: &Path) -> Result<(), Outcome> {
    match outcome(path) {
        Outcome::Run => Ok(()),
        outcome => Err(outcome),
    }
}
