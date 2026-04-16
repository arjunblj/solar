---
name: cargo-build
description: Compile the Solar workspace. The fastest "did my change break the build" oracle. Always run first before touching tests.
tier: prerequisite
---

# cargo-build

## When

- Before any other oracle.
- After adding or moving a crate, or editing Cargo.toml.
- After a rebase of the fork onto upstream.

## Canonical invocation

```bash
cargo build --workspace
```

Expected exit code: `0`.

## Test case

From a clean checkout of `arjunblj/solar`:

```bash
cargo build --workspace 2>&1 | tail -5
```

Should produce a `Finished` line and exit 0.

## Notes

- Do not substitute `cargo check` for this skill. `check` misses codegen
  failures in proc-macros.
- If the build breaks on a dependency you didn't touch, check `Cargo.lock`
  drift against upstream before "fixing" it.
