---
name: revm-deploy
description: Deploy a contract into revm and invoke it, comparing storage / returndata / reverts against solc's output. End-to-end runtime oracle for the codegen track.
tier: advisory
required_track: codegen-mir
---

# revm-deploy

## When

- Any `codegen-slice` or `mir-pass-slice` PR that changes emitted bytecode.
- When extending `corpus.execution` coverage.
- **Not applicable** until `crates/codegen` exists in this fork. Until then
  this skill is **reference-only**.

## Canonical invocation

Once codegen lands:

```bash
# Compile with solc
solc --optimize --bin Contract.sol > /tmp/solc.bytecode

# Compile with solar
cargo run --release -- --emit bytecode Contract.sol > /tmp/solar.bytecode

# Deploy + invoke both in revm via tools/diff-harness
cargo run -p diff-harness -- deploy /tmp/solc.bytecode /tmp/solar.bytecode --calldata 0x...
```

## Expected output

```
{
  "storage_match": true,
  "returndata_match": true,
  "reverts_match": true,
  "gas_solc": 21456,
  "gas_solar": 21472,
  "gas_ratio": 1.0007
}
```

## Test case

When tooling exists, smoke-test against `corpus.execution`:

```bash
cargo run -p diff-harness -- corpus --budget 60s
```

## Notes

- Strip the CBOR metadata trailer before bytecode-equivalence comparisons
  (solc emits compiler version + metadata hash; Solar will differ).
- Track `bytecode-equivalence` (#704) is the home for this harness.
- Do NOT claim codegen progress on the basis of "it compiles and a unit
  test passes". Require a revm-deploy run.
