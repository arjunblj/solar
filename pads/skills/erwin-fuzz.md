---
name: erwin-fuzz
description: Run the Erwin grammar-based differential fuzzer against Solar. Uncovers parser / sema divergences vs solc/solang/solar. Continuous, cheap, infinite-supply bug source.
tier: advisory
---

# erwin-fuzz

## When

- Continuous CI (nightly or per-hour job).
- After a parser change that expands grammar coverage.
- When looking for the next tight regression fixture.

## Canonical invocation

From the repo root:

```bash
./fuzz/run_erwin.sh --target solar --time-budget 1h
```

Output: a list of minimized fixtures that either crash Solar or produce
differential divergences vs solc.

## Test case

```bash
./fuzz/run_erwin.sh --target solar --time-budget 30s --smoke
```

Should exit 0 and produce a small `fuzz/erwin-out/` directory.

## Notes

- Every divergence found is a potential regression-test PR for track
  `testing-infra` (archetype `test-port`).
- Crashes with `I-ICE` label auto-open a regression fixture in a follow-up PR.
- Erwin initial-run research (arXiv:2503.20332) found 26 bugs across
  solc/solang/solar. Treat as a bug-finding machine, not a benchmark.
