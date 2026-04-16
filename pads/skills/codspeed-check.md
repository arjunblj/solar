---
name: codspeed-check
description: Run Solar benchmarks through CodSpeed + iai-callgrind. Advisory-tier oracle; detects perf regressions in parser/lexer/sema hot paths.
tier: advisory
---

# codspeed-check

## When

- Any change touching parser, lexer, sema, or IR hot paths.
- Before merging a performance-track PR.
- As a gate only when the change is explicitly labeled `perf-sensitive`.

## Canonical invocation

```bash
cargo codspeed build
cargo codspeed run
```

CodSpeed will post a delta in the PR thread.

## Budget

Auto-draft a PR whose CodSpeed delta is worse than **-5%**. The
`pr_rubric.must_include` block requires `CodSpeed delta` for any
perf-sensitive PR.

## Test case

```bash
cargo bench -p solar-bench --bench criterion -- --quick 2>&1 | tail -20
```

Should complete and report per-benchmark timings.

## Notes

- Do not gate on CodSpeed delta for non-perf-sensitive PRs. It adds noise
  and slows review.
- For non-hot-path changes, the criterion `--quick` run (above) is enough
  evidence; a full CodSpeed run is overkill.
- Cite the CodSpeed run URL in the PR body's Oracle evidence section.
