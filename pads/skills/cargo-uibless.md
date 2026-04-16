---
name: cargo-uibless
description: Regenerate UI test snapshots. Dangerous; requires manual inspection of every resulting diff. Never a shortcut to make CI green.
tier: caution
---

# cargo-uibless

## When

- A diagnostic was **deliberately** changed and the corresponding `.stderr`
  file needs to be regenerated.
- You have already read the new snapshot diff and it is correct.

## When NOT

- CI is red and you want it green. That is a reward hack. Fix the source.
- A snapshot changed but you don't understand why. Understand first.
- `testdata/solidity/**` — that submodule is human-managed; never bless
  against it.

## Canonical invocation

```bash
cargo uibless
```

After running, review the diff:

```bash
git diff tests/ui/
```

## Test case

On a fresh checkout where all UI tests pass:

```bash
cargo uibless && git diff --exit-code tests/ui/
```

Exit code should be 0 — no snapshots should change if none of the source
changed.

## Guardrail

The reward-hack pre-flight linter will block any integration diff that
edits `.stderr` files without a matching source edit. This skill's output
is expected to appear in the same commit as the source change that caused it.
