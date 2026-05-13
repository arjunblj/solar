# Solar Oracle Rules

Use the cheapest oracle that proves the claim, but do not overclaim beyond it.

## Current runnable gates

- `cargo +nightly fmt --all --check`: formatting only. This never proves semantic correctness.
- `typos --format brief`: spelling and identifier lint only.
- `cargo check --workspace`: Rust type/build surface.
- `cargo build --workspace`: build surface.
- `cargo clippy --workspace --all-targets -- -D warnings`: Rust lint gate.
- `cargo nextest run --workspace`: workspace tests.
- `cargo test --doc --workspace`: doctests.
- `cargo uitest`: Solar UI diagnostics and expected output fixtures.
- `TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests`: parser-oriented Solidity corpus pressure. This is not a TypeError/typeck parity proof.
- `TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests`: Yul corpus pressure. This does not prove Solidity HIR compatibility.

## Future or conditional oracles

- TypeError/typeck parity requires a distinct lane using `-Ztypeck`, explicit fixture IDs, and non-ignored exit semantics.
- Standard JSON parity requires a Solar Standard JSON front door before artifact parity claims.
- Foundry claims require a selected real fixture/project with `foundry.toml`, remappings, and build-info capture.
- Bytecode/runtime claims require Solar bytecode output plus normalized solc comparison or runtime-equivalence evidence.

## Proof discipline

Every PR should name:

- exact command,
- exit code,
- solc version when solc is involved,
- fixture IDs or corpus family,
- strongest passing tier,
- unsupported fields or checks not run,
- what the evidence does not prove.
