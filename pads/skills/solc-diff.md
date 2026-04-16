---
name: solc-diff
description: Compare Solar output against `solc` for a specific fixture. The ground-truth oracle for "does my change match Solidity semantics".
tier: gate
---

# solc-diff

## When

- Any `sema-slice` / `typeck-slice` / `diagnostic-port` PR.
- When porting a solc error code (NNNN) into Solar.
- When closing a `-Ztypeck` gap.

## Canonical invocation

Pin to `solc 0.8.31`:

```bash
solc --bin --ast-compact-json tests/ui/typeck/<slug>.sol > /tmp/solc.out
cargo run --release -- --emit ast-json tests/ui/typeck/<slug>.sol > /tmp/solar.out
diff /tmp/solc.out /tmp/solar.out
```

## Test case

For `tests/ui/typeck/basic.sol`:

```bash
solc --version  # should report 0.8.31
cargo run --release -- --emit ast-json tests/ui/typeck/basic.sol | head -20
```

## Notes

- The corpus runner in `tools/tester` has modes `Mode::Ui`,
  `Mode::SolcSolidity`, `Mode::SolcYul` — use those for bulk comparisons,
  not this skill.
- This skill is for one-file focused checks during development.
- For ABI + bytecode diffing, see `solc-abi-diff` (not yet implemented in
  this fork; tracked under track `bytecode-equivalence`).
