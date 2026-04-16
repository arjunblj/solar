---
name: tools-tester-solc-mode
description: Run the Solar tester against solc's pinned test corpus. Bulk correctness oracle for typeck coverage; mirrors upstream PR #737.
tier: gate
---

# tools-tester-solc-mode

## When

- After closing a typeck TODO, to measure corpus pass-rate delta.
- When porting a test category from `testdata/solidity/test/libsolidity/syntaxTests/<category>/`.
- As part of the `testing-infra` track.

## Canonical invocation

```bash
TESTER_MODE=solc-solidity cargo nextest run -p solar-tester
```

Or for Yul:

```bash
TESTER_MODE=solc-yul cargo nextest run -p solar-tester
```

## Before/After measurement

The pr_rubric `Before/After` block requires reporting corpus counts:

```
- solc syntaxTests: 2847 / 3551 passing (was 2840 / 3551, +7)
- solc yulTests:    402 / 357 passing  (unchanged)
```

## Test case

```bash
TESTER_MODE=solc-solidity cargo nextest run -p solar-tester --no-fail-fast 2>&1 | grep -E 'passed|failed' | tail -5
```

## Notes

- The corpus runner is primarily a **parser/corpus oracle** unless explicit
  `-Ztypeck` routing is enabled for the relevant semantic family. Do not
  over-claim what `TESTER_MODE=solc-solidity` proves on its own.
- Every removed entry from `tools/tester/src/solc/solidity.rs` skip-list is
  a landed-PR metric. Shrinking that skip-list is the public progress signal.
- See `solc-diff` skill for one-file focused comparisons.
