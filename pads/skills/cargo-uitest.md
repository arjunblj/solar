---
name: cargo-uitest
description: Run Solar's UI tests with `//~ ERROR/WARN/NOTE/HELP` annotations against `tests/ui/**` fixtures. The primary diagnostics oracle.
tier: gate
---

# cargo-uitest

## When

- Any diagnostic change in `crates/interface/src/diagnostics/**`.
- Any sema / typeck change that affects emitted errors.
- Before shipping a `diagnostic-port` or `sema-slice` PR.

## Canonical invocation

```bash
cargo uitest
```

Expected exit code: `0`.

## Test case

Add a new fixture under `tests/ui/<track>/<slug>.sol` with `//~ ERROR: TypeError NNNN` annotations, then:

```bash
cargo uitest 2>&1 | grep -E '(passed|failed)' | tail -3
```

Should show the new fixture as passing.

## Notes

- See `cargo-uibless` skill before considering a snapshot update. Do not
  bless without understanding what changed.
- UI fixtures live next to the error code they exercise. Prefer one fixture
  per error code over bundling.
- AGENTS.md diagnostic style rules apply: no trailing periods, backticks
  for code references, `note`/`help`/`span_note` subdiagnostics.
