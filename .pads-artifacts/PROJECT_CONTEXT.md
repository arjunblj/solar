# solar

> Build Solar into a serious alternative to solc — a Rust Solidity compiler with correct frontend semantics, meaningful type-checking parity, high-quality diagnostics, competitive performance, and eventually full MIR + EVM codegen with runtime-verifiable output. Push beyond the published roadmap into 2024–2026 compiler SOTA (aegraphs, translation validation, superoptimization, metamorphic fuzz, LLM rewrite-rule synthesis) where it makes sense.


## Objective
- Metric: Not yet synthesized (satisfy)
- Quality gate: hybrid
- Current best: No best result captured yet


## Setup
```sh
git clone https://github.com/arjunblj/solar.git && cd solar
git clone https://github.com/arjunblj/solar.git && cd solar


# test
cargo test

# lint
cargo clippy --workspace --all-targets
```

## Repository
- URL: https://github.com/arjunblj/solar
- Clone: `git clone https://github.com/arjunblj/solar.git && cd solar`

## Workstreams
- Type checker (high)
  scope: crates/sema/src/typeck/**
- Yul → HIR lowering (high)
  scope: crates/parse/src/yul/**, crates/sema/src/hir/**
- Testing infra + oracles (high)
  scope: tools/tester/**, tests/**, fuzz/**
- Diagnostic quality + parity (medium)
  scope: crates/interface/**, crates/sema/src/**
- MIR + EVM codegen (high)
  scope: crates/codegen/**, crates/mir/**
- Bytecode equivalence harness (high)
  scope: tools/diff-harness/**
- Performance (medium)
  scope: crates/**, benches/**
- Fuzzing + hardening (low)
  scope: fuzz/**
- LSP (low)
  scope: crates/lsp/**
- Novel SOTA research (low)
  scope: research/**

## Verification
- Quality gate: hybrid
- Test command: cargo test
- Lint command: cargo clippy --workspace --all-targets
- Additional commands: cargo check --workspace · cargo build --workspace · cargo nextest run --workspace · cargo uitest · cargo clippy --workspace --all-targets -- -D warnings · cargo +nightly fmt --all --check · typos --format brief · TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests · TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests · cargo test -p solar-tester · cargo codspeed build && cargo codspeed run


## Context
- Architecture: Owned baseline: arjunblj/solar@main (10283ae5c165) | Entrypoints: .github/scripts/install_iai_callgrind_runner.sh, benches/analyze/main.py | Config files: Cargo.toml, benches/Cargo.toml, examples/Cargo.toml, benches/analyze/pyproject.toml, crates/ast/Cargo.toml, crates/cli/Cargo.toml, crates/config/Cargo.toml, crates/data-structures/Cargo.toml | Important paths: .pads/spec.json, PADS.md, AGENTS.md, CLAUDE.md, README.md, Cargo.toml, benches/README.md, .github/workflows/ci.yml
- Key issues: Owned implementation baseline is arjunblj/solar@main (10283ae5c165)., Use paradigmxyz/solar docs, issues, merged history, and open PRs as reference material only. Do not assume upstream unmerged code exists in this fork.
- Reference docs: .pads/spec.json, PADS.md, AGENTS.md, CLAUDE.md, README.md, Cargo.toml, benches/README.md, .github/workflows/ci.yml
- Notes: Loaded 8 repository context files for grounding. Upstream references from paradigmxyz/solar are guidance only and must not be treated as code already present in this fork.

# Solar Completion Program

This document is the operating constitution for a long autonomous run whose goal is to complete Solar as a production-grade Solidity compiler. It is not a list of tiny starter edits. It tells the orchestrator how to reason, what to prove, where to look upstream, how to phase the work, and which evidence makes a PR worth opening.

The run should start with no user mission. The mission is this file.

## North Star

Solar becomes serious when it can compile real Solidity projects with solc-compatible behavior, expose a maintainable Rust compiler architecture, and produce upstreamable PRs whose evidence is strong enough that a Paradigm reviewer can merge them without asking basic follow-up questions.

Correctness comes first. Performance is only meaningful once the compiler is proving the right behavior against solc, production contracts, and eventually runtime execution.

## Orchestrator Handoff

The orchestrator should treat this file as the mission and planning brief. Do not rely on an external run prompt. Do not optimize for "opened a PR." Optimize for a multi-month sequence of upstreamable changes that monotonically increases measured correctness.

The first no-mission run should:

1. Bootstrap the repo with `.pads/setup.sh`.
2. Read this `PADS.md`, `AGENTS.md`, upstream `paradigmxyz/solar` issues/PRs/branches, and local fork diffs.
3. Seed the frontier from `starter_tasks`, but refine them against current repo evidence before dispatching implementers.
4. Start in Phase 0 and Phase 1. Do not dispatch codegen, bytecode-equivalence, or performance implementation work until the relevant correctness harnesses exist.
5. Prefer fewer, larger, real PRs over tiny motion PRs. A meaty PR can be hundreds or thousands of lines if it is dependency-sliced, tested, and reviewable.
6. Keep branch-import work explicit: reference branches first, integration branches second, cherry-picks only when small and tested.
7. Preserve context. Every completed or blocked task should leave behind a frontier entry that names the next dependency, oracle, and files.

The planner has freedom to choose the exact next slice. The boundaries are: correctness before performance; reference upstream but write only to this fork; every behavioral claim needs an oracle; and every PR should move Solar measurably closer to production Solidity compatibility.

## Current State

The fork is essentially upstream `main` plus Pads guardrails. Treat local fork state as authoritative. Treat `paradigmxyz/solar` issues, PRs, and branches as reference material unless explicitly imported into this fork.

Mainline Solar today is mostly frontend:

```text
lexer -> parser -> AST -> HIR -> sema/typeck -> diagnostics/ABI output
```

What exists:

- `solar-parse`: lexer, parser, Yul parser, parser diagnostics.
- `solar-ast`: AST definitions, visitors, JSON-ish emission surfaces.
- `solar-sema`: resolver, semantic analysis, gated type checker.
- `solar-interface`: source map, diagnostics, sessions.
- `solar-cli`, `solar-config`, `solar-macros`, `solar-data-structures`.
- `tools/tester`: UI and solc corpus runner.
- `benches`: Criterion, CodSpeed, and iai-callgrind benchmark surfaces.

What is not yet mainline-complete:

- Broad `-Ztypeck` solc corpus exposure and semantic parity.
- NatSpec and frontend emission parity.
- Inline assembly / Yul lowering into HIR.
- MIR, HIR-to-MIR lowering, EVM codegen, stack scheduling, assembler.
- Bytecode equivalence harness.
- Runtime equivalence through Foundry/Anvil/revm.
- LSP built on stable typeck.
- Production-corpus correctness and performance gates.

## Phase Model

The orchestrator should keep this ordering unless a blocker forces preparatory work.

### Phase 0: Setup, Import Intelligence, And Measurement

Goal: make every future claim measurable.

Required outcomes:

- Sandbox setup reliably initializes Rust, `cargo-nextest`, `typos`, PADS guards, and the pinned `testdata/solidity` submodule.
- The run imports upstream issue/branch intelligence into the Pads frontier.
- Corpus commands point at the correct package and test target.
- Production corpora are cloned, cached, and smoke-compiled in a way the orchestrator can shard later.
- The first PRs may be harness/setup PRs, but they must unlock stronger evidence.

Do not skip this phase. If the harness cannot measure correctness, implementation PRs will become motion.

### Phase 1: Frontend Correctness

Goal: make Solar credible on parse, diagnostics, AST, ABI, NatSpec, and semantic/type checking before claiming full compiler correctness.

Workstreams:

- Parser and diagnostics parity against owned UI fixtures and solc parser corpus.
- Type checker parity under `-Ztypeck`, centered on `paradigmxyz/solar#615`, `#663`, and PR `#737` as reference.
- Diagnostic quality ports from solc error codes into Solar `error_code!(NNNN)` plus fixtures.
- AST / ABI / NatSpec Standard JSON parity for selected contracts.
- Skip-list reduction in `tools/tester/src/solc/solidity.rs` and `tools/tester/src/solc/yul.rs`.

Representative meaty PRs:

- Enable one category of solc TypeError tests under `-Ztypeck`, keep counts, and port owned UI fixtures for the first gaps.
- Implement argument-aware overload resolution with fixtures for ambiguous calls, named args, and member overloads.
- Implement call-kind parity for array `push`, call options, and `require(cond, CustomError(...))`.
- Add an AST/NatSpec parity harness for a small stable corpus and fix the first local emission mismatch.

### Phase 2: Yul, HIR, MIR, Codegen, And Runtime Correctness

Goal: get from frontend correctness to executable EVM output with a proof path.

Reference branch:

- `paradigmxyz/solar:feat/codegen-mir` / PR `#693` is the major codegen branch. It is not a blind merge target. It is a reference branch to extract architecture and reviewable slices from.
- PR `#749` was merged into `feat/codegen-mir`, not `main`. Account for that branch reality.

Dependency order:

1. Yul/inline assembly to HIR boundary (`#415`). Prefer the branch's dedicated `hir::yul` direction as reference, but land reviewable slices.
2. MIR-only infrastructure, text format/parser, validator, and pass manager.
3. HIR-to-MIR lowering for arithmetic, locals, branches, loops, returns, and minimal builtins.
4. Liveness (`#694`) and phi elimination (`#695`).
5. Stack model <=16 (`#696`), then full stack scheduling/spilling (`#697`).
6. Assembler label/jump resolution (`#698`).
7. First executable bytecode.
8. Bytecode equivalence harness (`#704`) with Foundry/Anvil/revm.
9. Complex types, storage, calldata, ABI, events, constructors, calls (`#699` and branch commits).
10. Optimizer passes only after runtime oracles exist: DCE (`#700`), constant folding (`#701`), SCCP (`#702`), CSE (`#703`).

Do not claim codegen progress unless the PR states exactly which runtime/equivalence tier is available and which is still missing.

### Phase 3: Production Solidity Correctness

Goal: compile real Solidity projects, not just fixtures.

Corpus phases:

- Phase 3A: `forge-std`, small Foundry fixture projects, direct single-file remapping smoke.
- Phase 3B: OpenZeppelin, Solmate, PRBMath.
- Phase 3C: Solady, Uniswap v3, Uniswap v4.
- Phase 3D: Aave v3, Maker DSS, Compound v2/v3.
- Phase 3E: EigenLayer and other large modern Foundry projects once sharding/caching are stable.

Use framework builds to test project layout, remappings, build-info, profiles, and import resolution. Use direct Standard JSON inputs to compare solc and Solar outputs. Do not use production corpora as a first PR gate for small parser fixes, but always move serious correctness claims toward these corpora.

### Phase 4: Performance

Goal: preserve and improve Solar's speed after correctness has a floor.

Method:

- Profile before optimizing.
- Use Reth-style and Foundry-style practice: CodSpeed, Criterion, iai-callgrind, flamegraph/Samply, DHAT, Cachegrind/Callgrind, and macro real-project corpora.
- Optimize one hot path per PR.
- Include correctness oracle plus performance oracle in every performance PR.

Likely hot paths:

- Lexer cursor and tokenization.
- Parser token advancement and expression parsing.
- Unicode/string/unescape handling.
- Source map and source file creation.
- Symbol interning.
- AST arena allocation and thin slices.
- HIR lowering.
- Name resolution and type checking.
- Import graph and file IO.

## Correctness Oracle Ladder

Passing a lower tier only proves that tier. Do not overclaim.

| Tier | Oracle | What It Proves |
| --- | --- | --- |
| 0 | `cargo +nightly fmt --all --check`, `typos --format brief` | style hygiene |
| 1 | `cargo check --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` | Rust compile/lint health |
| 2 | `cargo nextest run --workspace`, `c