---
pads_version: 2
preset: compiler

tier0:
  project:
    slug: arjunblj/solar
    mission: >
      Build Solar into a serious alternative to solc — a Rust Solidity
      compiler with correct frontend semantics, meaningful type-checking
      parity, high-quality diagnostics, competitive performance, and
      eventually full MIR + EVM codegen with runtime-verifiable output.
      Push beyond the published roadmap into 2024–2026 compiler SOTA
      (aegraphs, translation validation, superoptimization, metamorphic
      fuzz, LLM rewrite-rule synthesis) where it makes sense.
    upstream: { full_name: paradigmxyz/solar, policy: reference_only }
    baseline_commit: 3a06d90

  hard_constraints:
    - Never edit files under testdata/solidity/ — pinned community submodule.
    - Never edit .stderr/.stdout snapshots without a matching source change.
    - Never cargo uibless without inspecting and justifying every diff.
    - Never modify .github/workflows/ or deny.toml/clippy.toml/rustfmt.toml without human review.
    - Never bump the rustc MSRV or add a top-level workspace dependency without human review.
    - Never claim codegen/runtime progress without a runtime differential oracle.
    - Never assume upstream paradigmxyz/solar unmerged work is present in this fork.

  scope_of_autonomy:
    permitted_subgoals:
      - advance any active track
      - port solc syntax tests into tests/ui/
      - build production Solidity correctness corpora
      - import or cherry-pick upstream branches after explicit branch analysis
      - open PRs within preset blast-radius caps
      - land research prototypes under research/
    permitted_side_effects:
      max_files_per_pr: 20
      max_crates_per_pr: 2
      max_wall_time_per_task_min: 120
      max_api_spend_per_task_usd: 20
    must_pause_for_approval:
      - .github/workflows/**
      - testdata/solidity/**
      - new crate, new IR, or new pass pipeline proposals
    shutdown_timer: { wall_clock_days: 90 }

  edit_rules:
    - PADS.md is the source of truth; .pads/spec.json is only a generated mirror.
    - Tier-0 is human-only. Requires atomic update of PADS.md + generated .pads/spec.json + .pads/tier0.sha256.
    - Tier-1 edits need critic-agent review + human approval.
    - System memory (lessons, rejected ideas, rules, status) lives in the pads platform, not in this repo.

oracles:
  - { id: cargo.check,           kind: shell, tier: prerequisite, command: "cargo check --workspace" }
  - { id: cargo.build,           kind: shell, tier: prerequisite, command: "cargo build --workspace" }
  - { id: cargo.nextest,         kind: shell, tier: gate,         command: "cargo nextest run --workspace" }
  - { id: cargo.uitest,          kind: shell, tier: gate,         command: "cargo uitest" }
  - { id: cargo.clippy,          kind: shell, tier: gate,         command: "cargo clippy --workspace --all-targets -- -D warnings" }
  - { id: cargo.fmt,             kind: shell, tier: gate,         command: "cargo +nightly fmt --all --check" }
  - { id: typos,                 kind: shell, tier: prerequisite, command: "typos --format brief" }
  - { id: solc_syntax_parser,    kind: shell, tier: gate,         command: "TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests" }
  - { id: solc_yul_parser,       kind: shell, tier: gate,         command: "TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests" }
  - { id: solar_tester_unit,     kind: shell, tier: gate,         command: "cargo test -p solar-tester" }
  - { id: codspeed_check,        kind: shell, tier: advisory,     command: "cargo codspeed build && cargo codspeed run" }

corpora:
  - id: forge-std
    source: foundry-rs/forge-std
    commit: pin-after-first-fetch
    phase: 3A
    setup: ["forge --version", "git rev-parse HEAD", "forge config --json", "forge remappings", "forge build --force --build-info --build-info-path out/build-info"]
    proves: ["Foundry project layout", "remappings", "basic production imports"]
    does_not_prove: ["Solar correctness", "Standard JSON parity", "runtime equivalence"]
  - id: openzeppelin-contracts
    source: OpenZeppelin/openzeppelin-contracts
    commit: pin-after-first-fetch
    phase: 3B
    setup: ["npm ci", "npm run compile", "extract artifacts/build-info/*.json"]
    proves: ["inheritance", "interfaces", "proxies", "custom errors", "storage layout"]
    does_not_prove: ["Solar correctness until Solar can replay Standard JSON inputs"]
  - id: solady
    source: Vectorized/solady
    commit: pin-after-first-fetch
    phase: 3C
    setup: ["forge build --force --build-info --build-info-path out/build-info"]
    proves: ["inline assembly", "modern opcodes", "gas-sensitive library code"]
    does_not_prove: ["Solar runtime correctness until Solar emits bytecode and a runtime oracle passes"]
  - id: prb-math
    source: PaulRBerg/prb-math
    commit: pin-after-first-fetch
    phase: 3B
    setup: ["forge build --force --build-info --build-info-path out/build-info"]
    proves: ["UDVTs", "fixed-point arithmetic", "custom errors"]
    does_not_prove: ["Solar correctness until Standard JSON replay exists"]
  - id: uniswap-v4-core
    source: Uniswap/v4-core
    commit: pin-after-first-fetch
    phase: 3C
    setup: ["forge build --force --build-info --build-info-path out/build-info"]
    proves: ["modern Foundry profiles", "hooks", "transient-storage-era code", "viaIR pressure"]
    does_not_prove: ["Solar runtime correctness until Solar bytecode can be compared and executed"]

branch_policy:
  default_upstream_mode: reference_only
  write_target: fork_only
  require_import_decision: true
  watchlist:
    - id: codegen-mir
      source: paradigmxyz/solar
      upstream_ref: feat/codegen-mir
      mode: extract
      priority: critical
      related: ["paradigmxyz/solar#693", "paradigmxyz/solar#749", "paradigmxyz/solar#687", "paradigmxyz/solar#704"]
      notes: "Reference branch for MIR/codegen architecture; never blind merge."
    - id: typeck-solc-corpus
      source: paradigmxyz/solar
      upstream_ref: pull/737/head
      mode: port
      priority: high
      related: ["paradigmxyz/solar#615", "paradigmxyz/solar#663", "paradigmxyz/solar#737"]
      notes: "Reference for exposing TypeError solc corpus under -Ztypeck."
    - id: sema-caches
      source: paradigmxyz/solar
      upstream_ref: pull/754/head
      mode: track
      priority: medium
      related: ["paradigmxyz/solar#754"]
      notes: "Perf-oriented sema cache work; track until correctness gates are stable."
    - id: this-shadowing-fix
      source: paradigmxyz/solar
      upstream_ref: pull/755/head
      mode: cherry_pick
      priority: high
      related: ["paradigmxyz/solar#216", "paradigmxyz/solar#755"]
      notes: "Small current sema bugfix candidate; preserve tests/authorship if imported."

tracks:
  - { id: typeck,               name: Type checker,                 priority: high,     status: active, scope: ["crates/sema/src/typeck/**"], upstream_tracking: ["paradigmxyz/solar#615"] }
  - { id: yul-hir,              name: Yul → HIR lowering,           priority: high,     status: active, scope: ["crates/parse/src/yul/**", "crates/sema/src/hir/**"], upstream_tracking: ["paradigmxyz/solar#415"] }
  - { id: testing-infra,        name: Testing infra + oracles,      priority: high,     status: active, scope: ["tools/tester/**", "tests/**", "fuzz/**"] }
  - { id: diagnostics,          name: Diagnostic quality + parity,  priority: medium,   status: active, scope: ["crates/interface/**", "crates/sema/src/**"] }
  - { id: codegen-mir,          name: MIR + EVM codegen,            priority: high,     status: draft,  scope: ["crates/codegen/**", "crates/mir/**"], upstream_tracking: ["paradigmxyz/solar#687"] }
  - { id: bytecode-equivalence, name: Bytecode equivalence harness, priority: high,     status: draft,  scope: ["tools/diff-harness/**"], upstream_tracking: ["paradigmxyz/solar#704"] }
  - { id: performance,          name: Performance,                  priority: medium,   status: active, scope: ["crates/**", "benches/**"] }
  - { id: fuzz-hardening,       name: Fuzzing + hardening,          priority: low,      status: active, scope: ["fuzz/**"] }
  - { id: lsp,                  name: LSP,                          priority: low,      status: draft,  scope: ["crates/lsp/**"], upstream_tracking: ["paradigmxyz/solar#394"] }
  - { id: research-sota,        name: Novel SOTA research,          priority: research, status: active, scope: ["research/**"] }

priority_order: [testing-infra, diagnostics, typeck, yul-hir, codegen-mir, bytecode-equivalence, fuzz-hardening, performance, research-sota, lsp]

starter_tasks:
  - title: Import the active upstream branch map into the frontier
    description: >
      Inspect upstream paradigmxyz/solar branches and open PRs, especially
      feat/codegen-mir, onbjerg/lsp-scaffolding, #737, #754, and #755. Produce
      execution-ready slices that say whether to import, cherry-pick, track, or
      ignore each branch before writing code.
    expected_output: >
      A branch strategy artifact plus independent implementation tasks for any
      safe cherry-picks or branch-extraction work.
    track_id: testing-infra
    task_type: research
    reference_ids: ["paradigmxyz/solar#693", "paradigmxyz/solar#737", "paradigmxyz/solar#754", "paradigmxyz/solar#755"]
    reference_only: true
    verification_hint: "gh pr view / git compare evidence; no code changes"
  - title: Establish production Solidity correctness corpora
    description: >
      Add the first production-corpus harness slice for OpenZeppelin, Solady,
      PRBMath, forge-std, and Uniswap, using Foundry/build-info where possible
      and direct Standard JSON comparison where Solar supports it. Do not claim
      runtime correctness until Solar emits bytecode and the diff harness exists.
    expected_output: >
      One reviewable harness/corpus PR or a blocked artifact naming the exact
      missing Solar interface and the next implementation slice.
    track_id: testing-infra
    task_type: implementation
    reference_ids: ["OpenZeppelin/openzeppelin-contracts", "Vectorized/solady", "PaulRBerg/prb-math", "foundry-rs/forge-std", "Uniswap/v4-core"]
    verification_hint: "cargo check --workspace; focused corpus command documented in PR"
  - title: Expose more solc TypeError corpus through Solar typeck
    description: >
      Bring upstream #737's idea into this fork carefully: enable -Ztypeck for a
      narrow category of solc syntax tests, keep failures measurable, and port
      the first failing examples into owned tests/ui/typeck fixtures.
    expected_output: >
      A typeck/testing PR with before/after corpus counts and focused UI
      fixtures; or an execution-ready set of smaller follow-up tasks.
    track_id: typeck
    task_type: implementation
    reference_ids: ["paradigmxyz/solar#615", "paradigmxyz/solar#663", "paradigmxyz/solar#737"]
    verification_hint: "cargo uitest; TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests"
  - title: Extract the codegen dependency graph from feat/codegen-mir
    description: >
      Treat upstream feat/codegen-mir as a reference branch, not a blind merge.
      Identify the smallest sequence that lands MIR-only infrastructure,
      validation, HIR-to-MIR lowering, liveness, phi elimination, stack
      scheduling, and bytecode equivalence in reviewable order.
    expected_output: >
      A dependency graph plus the first importable PR slice; no codegen claim
      without a bytecode/runtime oracle plan.
    track_id: codegen-mir
    task_type: research
    reference_ids: ["paradigmxyz/solar#687", "paradigmxyz/solar#693", "paradigmxyz/solar#704"]
    reference_only: true
    verification_hint: "git compare feat/codegen-mir...main; cite commit hashes"
---

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
| 2 | `cargo nextest run --workspace`, `cargo test --doc --workspace` | unit/integration behavior |
| 3 | `cargo uitest` / `TESTER_MODE=ui cargo nextest run -p solar-compiler --test tests` | owned diagnostics/UI behavior |
| 4 | `TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests` | parser/corpus accept-reject behavior, not full typeck |
| 4Y | `TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests` | Yul parser corpus behavior |
| 5 | Standard JSON AST/ABI/NatSpec diff vs solc | frontend emission parity |
| 6 | Production Foundry/Hardhat build-info corpus compile | real project ingestion and frontend/type behavior |
| 7 | bytecode diff solc vs Solar | codegen output equivalence |
| 8 | Foundry/Anvil/revm runtime differential | deployed behavior equivalence |
| 9 | fuzz and metamorphic transforms | unknown-unknown discovery |
| 10 | SMT/translation validation for rewrites | proof-level optimizer correctness |

## Foundry Oracle Rules

Use Foundry when the claim is about project ingestion, remappings, build-info, Standard JSON compatibility, artifact shape, or runtime behavior through Forge tests. Do not use Foundry as a shortcut around Solar's own UI, solc corpus, Standard JSON, bytecode, or runtime gates.

Foundry oracle tiers:

| Tier | Command Shape | Proves | Does Not Prove |
| --- | --- | --- | --- |
| F0 | `forge build --build-info` with solc | corpus pins, project config, remappings, profiles, build-info capture | Solar correctness |
| F1 | `solar --base-path . $(forge remappings) <files>` | Solar can ingest Foundry-style imports and frontend files | Standard JSON, bytecode, runtime |
| F2 | `solar --standard-json < build-info-input.json` | solc-compatible Standard JSON input/output subset | deployed bytecode behavior unless bytecode outputs exist |
| F3 | `FOUNDRY_SOLC=.../solar forge build/test` | Foundry can invoke Solar as a compiler and consume artifacts | semantic equivalence outside tested paths |
| F4 | Solar-vs-solc Forge test comparison | runtime behavior for covered tests, gas/bytecode comparison under exact settings | full compiler equivalence |

Common corpus capture command for Foundry projects:

```bash
forge --version
git rev-parse HEAD
forge config --json
forge remappings
forge build --force --build-info --build-info-path out/build-info --ast --extra-output metadata --extra-output storageLayout
```

Before `--standard-json` exists on main, Foundry corpora are frontend pressure only:

```bash
cargo build --workspace
solar --base-path . $(forge remappings) $(git ls-files '*.sol' ':!:test/**' ':!:script/**') -Ztypeck
```

Once Standard JSON is extracted from `feat/codegen-mir`, replay build-info inputs:

```bash
jq '.input' out/build-info/<id>.json > /tmp/input.json
solc --standard-json < /tmp/input.json > /tmp/solc-output.json
solar --standard-json < /tmp/input.json > /tmp/solar-output.json
```

Once bytecode emit exists, compare Solar and solc through Foundry only with exact settings, fixed library addresses, and metadata normalization:

```bash
FOUNDRY_SOLC="$PWD/target/debug/solar" forge test --force --json -vvvvv --decode-internal --out out-solar --cache-path cache-solar
forge test --force --json -vvvvv --decode-internal --out out-solc --cache-path cache-solc
```

gakonst's comment on PR `#749` is the correctness target: the ideal harness compiles tests with solc, compiles only the contract-under-test with Solar, deploys Solar bytecode via `vm.deployCode` or an equivalent dynamic-link path, then compares return data, reverts, logs, state changes, and gas. `FOUNDRY_SOLC=solar` is a useful smoke test, not the final equivalence oracle.

## Upstream Policy

Always check upstream before selecting work.

Track immediately:

- `paradigmxyz/solar#693` / `feat/codegen-mir`: codegen architecture reference.
- `#737`: `-Ztypeck` solc corpus exposure reference.
- `#754`: sema cache performance work.
- `#755`: small sema shadowing fix for `this`/`super`.
- `#743`, `#744`: small parser/diagnostic fixes.
- `onbjerg/lsp-scaffolding` / `#401`: LSP reference, blocked behind typeck maturity.

Branch strategy:

- Import active reference branches into the fork only as tracking branches unless a task says otherwise.
- Cherry-pick small fixes only when the diff is narrow, current, and comes with or can receive tests.
- Do not merge `feat/codegen-mir` wholesale into `main`; extract a dependency-ordered integration branch and land reviewable slices.
- Ignore stale WIP branches unless a task cites an exact commit and a verification plan.

### gakonst / Foundry / Codegen Branch Policy

`paradigmxyz/solar:feat/codegen-mir` / PR `#693` is reference-only. It contains Standard JSON, Foundry fixtures, MIR, HIR-to-MIR, EVM codegen, stack scheduling, assembler, validators, pass-manager work, and optimizer passes. It also contains generated/noisy artifacts and broad side quests. Extract, do not merge.

Extraction order:

1. Standard JSON input/output surface for Foundry compatibility.
2. Minimal Foundry build-info replay fixtures.
3. MIR data model, text format, parser/printer, validator, and `solar-mir-opt`.
4. `--emit=mir` and MIR UI mode.
5. HIR-to-MIR lowering slices.
6. Stack scheduler, assembler, and minimal bytecode emit.
7. Solar-vs-solc bytecode diff harness.
8. Foundry runtime harness using mixed compilation, not just `FOUNDRY_SOLC=solar`.
9. Optimizer passes only after runtime gates exist.

Never import generated `out-*`, `cache-*`, vendored `forge-std` or `solmate` copies, benchmark images, broad workflow changes, or testdata submodule rewrites without human review. Every extraction PR must name source commit hashes, omitted commits, and the oracle tier it satisfies.

## Work Selection Rules

The planner should choose current repo-grounded slices, not pre-scripted tiny edits.

Good slices:

- A solc corpus category gets exposed under the correct Solar mode with counts.
- A specific TypeError parity gap gets an owned UI fixture and implementation.
- A parser/diagnostic mismatch gets a fixture, error code, and `cargo uitest`.
- A Yul statement kind gets lowered with a fixture and a clear blocked/unblocked dependency.
- A MIR/codegen infrastructure slice lands without claiming runtime parity.
- A production corpus harness adds one new measurable project or command.
- A performance PR changes one hot path and reports before/after benchmark evidence.

Bad slices:

- Docs-only work unless it unlocks run correctness.
- Comment-only or TODO-removal-only patches.
- Snapshot reblessing without source change and explanation.
- Codegen claims without bytecode/runtime oracle.
- Performance claims without profiling and correctness evidence.
- Broad "implement typeck" or "merge codegen" tasks without dependency slicing.

## Production Corpus Plan

Start small and make the harness stable before adding noisy protocols.

Phase A:

- `foundry-rs/forge-std`
- small Foundry fixtures
- direct one-file remapping smoke tests

Phase B:

- `OpenZeppelin/openzeppelin-contracts`
- `transmissions11/solmate`
- `PaulRBerg/prb-math`

Phase C:

- `Vectorized/solady`
- `Uniswap/v3-core`
- `Uniswap/v4-core`

Phase D:

- `aave/aave-v3-core`
- `sky-ecosystem/dss`
- `compound-finance/compound-protocol`
- `compound-finance/comet`

Phase E:

- `Layr-Labs/eigenlayer-contracts`

Each corpus PR must state which compiler versions, remappings, profiles, optimizer settings, `viaIR`, EVM version, metadata hash settings, and output selections it uses.

## PR Quality Bar

Every PR must include:

- Scope and changed files.
- Linked upstream issue/PR/branch when relevant.
- What behavior changed.
- Why this slice is in the current phase.
- Exact oracle commands and outputs.
- For corpus work: before/after counts.
- For solc parity: error code or Standard JSON field cited when relevant.
- For performance: baseline commit, benchmark commands, before/after numbers, and profiler summary.
- Known risks and follow-up tasks.

A task is not done because it compiles. It is done when the specific behavior is tested, the evidence is in the PR body, and the next frontier is clearer than before.

## House Style

Follow `AGENTS.md` and upstream style:

- Conventional commit titles.
- No trailing full stops in diagnostic messages.
- Backticks for code identifiers.
- Use `sym::name` or `kw::Keyword` rather than `.as_str()` where applicable.
- Visitors call `walk_*` unless intentionally stopping.
- Arena allocation discipline.
- UI annotations use `//~ ERROR:`, `//~ WARN:`, `//~ NOTE:`, `//~ HELP:`.

## Memory And Frontier Discipline

Before non-trivial work, search the Pads wiki and upstream issues/PRs. After work, record:

- what was learned,
- what failed,
- which corpus or oracle was decisive,
- exact follow-up slices,
- and whether the work belongs to Phase 1, 2, 3, or 4.

When blocked, do not keep reading. Produce a blocker artifact that names the missing dependency and creates the next actionable task.
