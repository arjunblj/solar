# solar-tester

Test runner for the Solar compiler.

This crate is invoked in `crates/solar/tests.rs` with the path to the `solar` binary.

It uses the `ui_test` framework to run tests located in the `tests/` directory.

## Oracle baseline output

Each tester mode prints deterministic `oracle-baseline:` lines before and after the
`ui_test` run. The start line includes the mode, corpus counts, and solc provenance:

```text
oracle-baseline: mode=solc-solidity status=running corpus_total=N corpus_included=N corpus_skipped=N solc_path=/path/to/solc solc_version=Version: 0.8.31+commit.example
```

When `SOLC` is not set for a solc corpus mode, the line says
`solc=missing(SOLC not set; corpus parser oracle does not invoke solc)`. These corpus
modes use solc test fixtures, but they do not invoke solc while checking Solar's parser.

The completion line is one of:

```text
oracle-baseline: mode=solc-yul status=clean reporter=ok top_failing_fixture_ids=[]
oracle-baseline: mode=solc-yul status=inherited-red reporter=ok top_failing_fixture_ids=[path/to/fixture.yul]
oracle-baseline: mode=solc-yul status=reporter-regression reporter=needs-investigation top_failing_fixture_ids=[]
```

`inherited-red` means Solar still fails fixtures in the existing corpus; the listed IDs
are the first deterministic failing fixture paths under that mode's corpus root.
`reporter-regression` means `ui_test` reported a failure but the baseline reporter could
not reproduce a corpus fixture mismatch, so the reporter or harness should be checked.
