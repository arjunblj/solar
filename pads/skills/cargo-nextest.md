---
name: cargo-nextest
description: Run the Solar unit test suite via nextest. Function-level correctness oracle. Parallel runner; faster and more deterministic than `cargo test`.
tier: gate
---

# cargo-nextest

## When

- After any source edit to `crates/**`.
- Before opening a PR.
- When the UI test suite has no relevant fixture for the change.

## Canonical invocation

```bash
cargo nextest run --workspace
```

Expected exit code: `0`.

## Filtered invocation

Run tests for a single crate:

```bash
cargo nextest run -p solar-sema
```

## Test case

```bash
cargo nextest run -p solar-sema --no-fail-fast 2>&1 | tail -10
```

Should report `summary: N tests passed, 0 failed`.

## Notes

- Do not bypass with `--no-fail-fast` to hide unrelated failures. Fix or
  skip the truly unrelated; fail-fast is the default for a reason.
- Flaky tests are a bug. Open an issue with a repro command; don't retry.
