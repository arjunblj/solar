# Solar Oracle Rules

Use the cheapest oracle that proves the claim, but do not overclaim
beyond it. Treat oracle tiers as **upper bounds** on what a PR may
assert: passing T2 unit tests does not prove T5 Standard JSON parity,
and passing T4 parser corpus does not prove T6 typecheck or T7 bytecode.

## Tier ladder (recap of `PADS.md` body)

| Tier | Proves | Does not prove |
| --- | --- | --- |
| T0 | fmt, typos, generated-file guard | semantic correctness |
| T1 | `cargo check` / `cargo build` / `cargo clippy` | runtime behavior |
| T2 | `cargo nextest run` / `cargo test --doc` | external compatibility |
| T3 | `cargo uitest` | solc parity |
| T4 | solc syntax / Yul corpus (parser-only) | typecheck or codegen behavior |
| T5 | Standard JSON frontend parity | runtime behavior |
| T6 | Foundry/Hardhat build-info replay | bytecode equivalence |
| T7 | normalized bytecode diff | runtime equivalence |
| T8 | revm/Anvil/Foundry runtime differential | absence of unknown bugs |
| T9 | fuzz / metamorphic | absence of bugs |
| T10 | SMT / proof / translation validation | modeled-rule correctness only |

## Current runnable gates

### Prerequisite (cheap, run early)

- `cargo +nightly fmt --all --check` — formatting only. Solar's
  `rustfmt.toml` uses unstable features (`imports_granularity`,
  `wrap_comments`, `format_macro_matchers`); stable rustfmt silently
  emits 100+ warnings and exits 1. The setup script pins nightly
  rustfmt as a hard install gate.
- `typos --format brief` — spelling / identifier lint only.
- `cargo check --workspace` — Rust type/build surface.
- `cargo build --workspace` — build surface.
- `python3 scripts/pads/spec-sync.py` — PADS.md ↔ spec.json sync.
- `python3 scripts/pads/tier0-guard.py` — Tier-0 hash integrity.
- `python3 scripts/pads/validate.py` — structural cross-references.

### Gate (correctness-blocking)

- `cargo clippy --workspace --all-targets -- -D warnings` — Rust lint
  gate.
- `cargo nextest run --workspace` — workspace tests.
- `cargo test --doc --workspace` — doctests.
- `cargo uitest` (alias for `cargo xtask test ui`) — Solar UI
  diagnostics and expected-output fixtures.
- `TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests`
  or `cargo tq solc-solidity` — parser-oriented Solidity corpus
  pressure. The tester uses `--stop-after=parsing`; this is **not** a
  TypeError / typeck parity proof.
- `TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests`
  or `cargo tq solc-yul` — Yul corpus pressure. Does **not** prove
  Solidity HIR compatibility.

### Advisory (do not gate; signal only)

- `cargo codspeed build && cargo codspeed run` — CodSpeed parser
  benches. Currently Solar-only; not a Solar-vs-solc gate.
- `cargo bench -p solar-bench --bench iai` — IAI instruction-count
  benches.
- `cargo bench -p solar-bench --bench criterion -- --quiet --format terse parser`
  — Criterion parser benches; uses `SOLC` env if set for
  Solar-vs-solc comparison.

## Future or conditional oracles

These are **proof contracts**, not yet runnable. They become gateable
only after the listed prerequisites land. Treat the corresponding
extensions.solar fields as the source of truth on status.

- **TypeError / typeck parity** requires a distinct lane using
  `-Ztypeck`, explicit fixture IDs, non-ignored exit semantics, and an
  xfail manifest with reason taxonomy. `TESTER_MODE=solc-solidity` is
  not it; that mode ignores TypeError exits.
- **Standard JSON parity** requires a Solar Standard JSON front door
  on fork main before artifact parity claims (PR #693 added one to
  the draft `feat/codegen-mir` branch; not on main).
- **Foundry claims** require a selected real fixture / project with
  `foundry.toml`, remappings, and build-info capture. Counting the
  fork repo as a Foundry project is wrong; pick a pinned fixture.
- **Bytecode / runtime claims** require Solar bytecode output plus
  normalized solc comparison or runtime-equivalence evidence. Until
  Solar emits bytecode for the selected subset on fork main, mark
  these lanes unavailable.
- **Fuzz regression gates** require a reducer/minimizer flow and durable
  minimized fixtures. Current fuzz scripts are discovery tools.
- **Coverage reports** are advisory until a repo-owned coverage command
  and CI policy exist.

## Verified test invocations

Aliases come from `.cargo/config.toml` and xtask wiring.

```bash
# UI tests (T3)
cargo uitest                           # alias: cargo xtask test ui
cargo uibless                          # alias: cargo xtask test ui --bless

# Snapshot tests (T2/T3)
SNAPSHOTS=overwrite cargo test -p solar-ast

# solc corpus modes (T4; parser-only)
cargo tq solc-solidity                 # alias: cargo xtask test solc-solidity
cargo tq solc-yul                      # alias: cargo xtask test solc-yul

# Equivalent nextest forms (also T4)
TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests
TESTER_MODE=solc-yul       cargo nextest run -p solar-compiler --test tests

# Bootstrap (idempotent)
bash .pads/setup.sh
bash .pads/setup.sh --check              # readiness probe; no installs

# Pinned solc for differential / perf (set externally)
export SOLC=/path/to/solc-0.8.31
```

Tester flags applied automatically: `-j1 --error-format=rustc-json
-Zui-testing -Zparse-yul`. solc corpus modes add `--stop-after=parsing`.

## Snapshot hygiene

Solar uses `snapbox` for snapshot tests and `ui_test` for UI tests.

- New / changed `snapbox` snapshots are written with
  `SNAPSHOTS=overwrite`.
- UI expected output is updated with `cargo uibless`.
- **Do not rebless or delete snapshots as cleanup** without paired
  source changes and a reviewed semantic before/after.
- `snapshots.clean` is a prerequisite oracle: a PR may not introduce
  uncommitted `.stderr` / `.stdout` / `.snap` drift unless it pairs
  the drift with a substantive source change.

## Performance phase report

Any PR claiming a performance change must produce a phase report (run
the `perf.phase-report` prerequisite oracle). At minimum the report
names: baseline command, head command, environment (CPU model, target
triple, Rust commit, Solar commit, solc version if relevant, EVM
version if relevant, corpus pin), metric set (median, p95, p99,
instructions, peak RSS), profile evidence naming the hot path, and
the correctness oracle that gates the optimized stage.

See `.pads/rules/performance.md` for the full no-claim ruleset and
prior-art ranking.

## CI baseline

`.github/workflows/ci.yml` aggregates: `test`, `feature-checks`,
`typos`, `clippy`, `docs`, `fmt`, `deny`. CodSpeed lives in
`.github/workflows/bench.yml` and is **not** part of the `ci-success`
aggregator. CodeQL is present but is also not a `ci-success`
dependency. Treat CodSpeed and CodeQL as advisory unless branch
protection promotes them.

CI does **not** download solc 0.8.31. The Solidity test corpus is the
submodule at `testdata/solidity` (URL `argotorg/solidity.git`, pinned
to 0.8.31). For Solar-vs-solc differential or perf claims, the
contributor / harness must export `SOLC=/path/to/solc-0.8.31` locally.

## Proof discipline

Every PR should name:

- exact command and exit code;
- solc version when solc is involved (default pin: `0.8.31`);
- EVM version when codegen / opcode behavior is involved;
- optimizer mode and runs when codegen is involved;
- fixture IDs or corpus family with commit pin;
- strongest passing tier;
- unsupported fields or checks not run;
- what the evidence does **not** prove;
- the next measured gap unlocked by this PR.

If any of the above is unknown, the PR is a measurement task, not a
compatibility claim.
