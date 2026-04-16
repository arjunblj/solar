---
name: alloy-abi-decode
description: Decode / encode ABI-encoded calldata and returndata using `alloy`. Utility skill for tests that inspect runtime semantics.
tier: utility
---

# alloy-abi-decode

## When

- Writing a `corpus.execution` test that asserts on concrete returndata.
- Debugging a codegen divergence when `revm-deploy` reports `returndata_match: false`.
- Inspecting a `solc` vs `solar` ABI disagreement.

## Canonical invocation

Rust:

```rust
use alloy::primitives::*;
use alloy::sol_types::SolValue;

let returndata: Bytes = ...;
let decoded = <(U256, Address)>::abi_decode(&returndata, true)?;
```

From the CLI (once `tools/diff-harness` exists):

```bash
cargo run -p diff-harness -- abi-decode \
  --types '(uint256,address)' \
  --data 0x000000000000000000000000000000000000000000000000000000000000002a0000000000000000000000001234...
```

## Expected output

```
(42, 0x1234...)
```

## Test case

```bash
cargo run -p diff-harness -- abi-decode --types 'uint256' --data 0x000...00002a
# → "42"
```

## Notes

- Use `alloy::sol_types` for strict decoding; use `alloy::primitives` types
  for raw bytes.
- Never trust ABI round-trips that skip the validation flag (`true` in
  `abi_decode`).
- For ABI encoding correctness vs solc, the `solc_abi_diff` oracle is the
  authoritative check. This skill is for local debugging only.
