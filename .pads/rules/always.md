---
alwaysApply: true
---

# Always-On Rules

These apply to every worker session regardless of which lane is active.

- All writes go to `arjunblj/solar:main`. Never open a PR against `paradigmxyz/solar`. Never push to it.
- Never edit `testdata/solidity/**` without explicit task approval. The pinned solc test corpus is read-only oracle input.
- Never edit `.github/workflows/**`, `rust-toolchain.toml`, `deny.toml`, `clippy.toml`, or `rustfmt.toml` without explicit task approval. These are protected configuration.
- Never grow skip / xfail lists without an issue link, reason code, owner track, and revisit condition.
- Never claim solc compatibility without naming the solc version, EVM version, optimizer mode, corpus, and strongest passing oracle tier.
- One invariant per PR. Pair every typeck/sema change with a paired UI fixture under `tests/ui/`. Production code lives in `crates/`; tests live alongside.
- Never edit or rebless UI snapshots (`*.stderr`, `*.stdout`, `*.snap`) without paired source changes and a reviewed semantic before/after.
- Never claim performance without a paired correctness oracle, same-environment baseline, before/after numbers, and profile evidence.
