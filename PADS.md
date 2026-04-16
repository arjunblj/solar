---
version: 1
mission: "Advance `arjunblj/solar` from its audited fork baseline (`d79be54` at planning time) into a complete, reviewable Solidity compiler roadmap program: improve frontend and type-checking correctness first, then build toward codegen, Yul, LSP, and optimization milestones while preserving Solar's performance identity and diagnostics quality."
metric: ""
metric_direction: satisfy
quality_gate: hybrid
contribution_policy: review_required

setup:
  run: "cargo run -- file.sol"
  test: "cargo nextest run --workspace && cargo uitest"
  lint: "cargo +nightly fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings"

verification:
  commands: ["cargo build --workspace","cargo nextest run --workspace","cargo uitest","cargo +nightly fmt --all --check","cargo clippy --workspace --all-targets -- -D warnings"]
  test_command: "cargo nextest run --workspace"
  lint_command: "cargo +nightly fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings"
  benchmark_command: "cargo bench -p solar-bench --bench criterion -- --quick"
  differential: {"reference":"`solc` 0.8.x behavior for Solidity semantics, diagnostics, ABI output, and eventually bytecode/runtime behavior","compare":["UI test snapshots in tests/ui/","solc corpus behavior through tools/tester","benchmark baselines in benches/","Foundry/revm downstream execution checks for codegen/runtime work"]}

tracks:
  - id: "typeck"
    name: "Type Checker Completion"
    description: "Expand `-Ztypeck` toward meaningful solc parity from the current fork baseline. Focus on missing semantic checks, UI tests, and corpus enablement."
    scope: ["crates/sema/src/typeck/**","crates/sema/src/ty/**","tests/ui/typeck/**","tools/tester/**"]
    depends_on: []
    priority: always
  - id: "frontend-correctness"
    name: "Frontend Correctness"
    description: "Parser, AST, lowering, sema, NatSpec, and diagnostics correctness work that is independent of MIR/codegen."
    scope: ["crates/parse/**","crates/ast/**","crates/sema/**","crates/interface/**","tests/ui/**"]
    depends_on: []
    priority: high
  - id: "diagnostics"
    name: "Diagnostics Quality"
    description: "Keep diagnostics precise, styled per AGENTS.md, and aligned with solc error code expectations where Solar aims for parity."
    scope: ["crates/interface/**","crates/sema/**","crates/parse/**","tests/ui/**"]
    depends_on: []
    priority: high
  - id: "testing-infra"
    name: "Testing Infrastructure"
    description: "Improve corpus enablement, differential checks, reproducible baselines, and execution oracles."
    scope: ["tools/tester/**","tests/**","testdata/**","benches/**","fuzz/**"]
    depends_on: []
    priority: high
  - id: "codegen"
    name: "MIR and Codegen"
    description: "Take over codegen work deliberately from the fork baseline. Upstream codegen PRs are reference material only until equivalent work is explicitly adopted into this fork. The current fork does not yet contain a `crates/codegen` crate."
    scope: ["crates/config/**","crates/cli/**","crates/sema/**","tools/tester/**","tests/**","testdata/**"]
    depends_on: ["typeck","testing-infra"]
    priority: high
  - id: "yul-hir"
    name: "Yul / Inline Assembly"
    description: "Lower Yul and inline assembly to HIR behind `-Zparse-yul`, with explicit tests and follow-on verification."
    scope: ["crates/parse/src/parser/**","crates/sema/src/hir/**","tests/ui/**"]
    depends_on: ["typeck"]
    priority: medium
  - id: "lsp"
    name: "Language Server Protocol"
    description: "Build editor-facing Solar functionality once type information and diagnostics are stable enough to support it. The current fork does not yet contain a dedicated `crates/lsp` crate, so early work should focus on adoption seams and minimal CLI/editor surfaces."
    scope: ["crates/cli/**","crates/sema/**","crates/interface/**","tests/**"]
    depends_on: ["typeck","diagnostics"]
    priority: medium
  - id: "performance"
    name: "Performance"
    description: "Protect and improve Solar's front-end performance identity without trading away correctness or maintainability."
    scope: ["crates/data-structures/**","crates/parse/**","crates/sema/**","benches/**"]
    depends_on: ["frontend-correctness"]
    priority: medium
  - id: "fuzz-hardening"
    name: "Fuzzing and Hardening"
    description: "Expand malformed-input resilience, crash resistance, and regression coverage."
    scope: ["fuzz/**","crates/parse/**","crates/sema/**","tests/**"]
    depends_on: ["frontend-correctness"]
    priority: low

constraints: ["Anchor all planning and completion tracking to the actual state of `arjunblj/solar`; upstream unmerged work is reference-only until adopted.","Follow AGENTS.md diagnostic style exactly: no trailing periods, backtick code references, and proper subdiagnostics.","Every typeck change must ship with focused UI tests and be checked against `solc` 0.8.31 behavior.","Do not use `cargo uibless` as a shortcut; inspect and justify every snapshot change.","Keep parser/sema/interface crates library-first; do not push core logic into the CLI layer.","Do not claim progress on codegen/runtime based only on successful compilation; use explicit downstream or differential oracles.","Treat `testdata/solidity/**` as readonly unless there is a deliberate reason to modify test fixtures at the fork level.","Prefer small, reviewable slices with tests over broad speculative refactors."]
editable_files: ["PADS.md","AGENTS.md","README.md","Cargo.toml",".cargo/config.toml","crates/**","tests/**","tools/**","benches/**","fuzz/**","examples/**","testdata/*.sol","scripts/**"]
readonly_files: [".github/**","LICENSE*","assets/**","testdata/solidity/**"]
anti_patterns: ["Assuming upstream unmerged PRs already exist in this fork.","Changing diagnostics wording or spans without corresponding UI test updates.","Claiming solc parity without naming the exact oracle used.","Using benchmarks as proof of correctness.","Running `cargo uibless` before understanding the semantic change.","Adding hot-path string allocations or `.as_str()` comparisons where interned symbols or keywords should be used.","Large codegen takeovers without a fork-baseline inventory and explicit adoption plan."]

starter_tasks:
  - title: "Inventory the fork baseline against the upstream roadmap"
    description: "Starting from the current `arjunblj/solar` baseline, map what is already present, what is missing, and what is only upstream reference material. Produce a concise report that names tracks, key files, and the highest-leverage near-term gaps."
    expected_output: "A structured baseline report anchored to the fork, with explicit references marked as `reference_only` where applicable."
    track_id: "testing-infra"
    task_type: "research"
    reference_ids: ["#1","#615","#687"]
    reference_only: true
    verification_hint: "Use repository inspection plus `cargo nextest run --workspace && cargo uitest` to verify the baseline is healthy."
  - title: "Audit typeck gaps against the fork baseline"
    description: "Compare the current fork type checker against the expected scope of issue `#615`. Focus on view/pure checking, remaining contract-level restrictions, and any gaps visible in UI tests or `solc` corpus coverage."
    expected_output: "A prioritized typeck gap list with concrete files, missing checks, and the exact UI or corpus tests that should verify each gap."
    track_id: "typeck"
    task_type: "research"
    reference_ids: ["#615","#737"]
    reference_only: true
    verification_hint: "Use `cargo uitest` and `TESTER_MODE=solc-solidity cargo test -p solar-compiler --test=tests`."
  - title: "Enable and measure more `-Ztypeck` corpus coverage"
    description: "Extend the tester so more upstream `TypeError`-style cases run through `-Ztypeck`, but treat upstream as a reference corpus only. Land the improvement in the fork with reproducible pass/fail reporting."
    expected_output: "Updated tester behavior, focused tests, and a fork-baseline report of what categories are now exercised."
    track_id: "testing-infra"
    task_type: "implementation"
    reference_ids: ["#663","#737"]
    reference_only: true
    verification_hint: "Run `cargo nextest run --workspace`, `cargo uitest`, and the relevant tester command."
  - title: "Implement a missing view/pure checker slice"
    description: "Choose one clearly missing state-mutability rule from the fork baseline, implement it behind `-Ztypeck`, and add focused UI tests with solc-aligned error codes."
    expected_output: "A small, reviewable typeck patch plus passing UI tests."
    track_id: "typeck"
    task_type: "implementation"
    reference_ids: ["#615"]
    reference_only: true
    verification_hint: "Run `cargo uitest` and a focused `solc` comparison for the changed cases."
  - title: "Audit diagnostics regressions and quick wins"
    description: "Identify the highest-signal diagnostics mismatches or open bugfixes in the fork baseline. Prefer small correctness wins such as precedence or warning handling fixes that can be verified cheaply."
    expected_output: "A short list of diagnostics fixes, each with the exact files, tests, and expected oracle."
    track_id: "diagnostics"
    task_type: "research"
    reference_ids: ["#743","#744"]
    reference_only: true
    verification_hint: "Use `cargo uitest` as the primary oracle."
  - title: "Map the next frontend-correctness slice from the fork baseline"
    description: "Identify one frontend completeness gap in the current fork that is narrower than a whole track audit, such as NatSpec lowering, ABI referenced-event coverage, parser edge cases, or other frontend-semantic mismatches. Name the exact files, tests, and oracle that would prove the slice."
    expected_output: "A concrete frontend-correctness slice proposal with one clear next implementation or verification task."
    track_id: "frontend-correctness"
    task_type: "research"
    reference_ids: ["#567","#305"]
    reference_only: true
    verification_hint: "Prefer focused UI tests and narrow compiler fixtures over broad whole-track inventory."
  - title: "Design the codegen takeover plan from the fork baseline"
    description: "Do not assume upstream codegen branches are present. Instead, analyze the current fork baseline, then map how upstream references like PR `#693` and issue `#687` could be adopted as explicit future slices."
    expected_output: "A takeover plan that separates fork state, upstream reference-only work, and first mergeable codegen milestones."
    track_id: "codegen"
    task_type: "research"
    reference_ids: ["#687","#693","#704"]
    reference_only: true
    verification_hint: "No codegen claims count as done without a named downstream oracle."
  - title: "Define downstream oracles for codegen/runtime work"
    description: "Specify the exact differential and execution checks that would be required before treating codegen progress as meaningful in this fork."
    expected_output: "A concrete verification design covering `solc`, Foundry, and revm/anvil-style execution checks."
    track_id: "testing-infra"
    task_type: "synthesis"
    reference_ids: ["#687","#704"]
    reference_only: true
    verification_hint: "Name exact commands, artifacts, and failure signals."
  - title: "Audit Yul readiness from the fork baseline"
    description: "Determine what the fork currently supports behind `-Zparse-yul`, what tests exist, and what the first good HIR-lowering slices would be."
    expected_output: "A fork-baseline Yul readiness report with candidate first implementation slices."
    track_id: "yul-hir"
    task_type: "research"
    reference_ids: ["#415","#652"]
    reference_only: true
    verification_hint: "Use focused UI tests and parser/lowering checks."
  - title: "Protect parser/sema performance while fixing correctness"
    description: "Identify one correctness-adjacent area where performance-sensitive rules need to remain explicit, and document the benchmark path to check after changes."
    expected_output: "A performance-aware change plan or small optimization with evidence."
    track_id: "performance"
    task_type: "experiment"
    reference_ids: ["benches/README.md"]
    reference_only: false
    verification_hint: "Use `cargo bench -p solar-bench --bench criterion -- --quick` when hot paths change."

context:
  architecture: "Solar is a Rust workspace compiler project. The owned baseline is `arjunblj/solar` on its default branch. Core pipeline today is lexer/parser -> AST -> HIR -> semantic analysis/typeck -> ABI or diagnostics output. Codegen, Yul lowering, and LSP are not assumed to exist in the fork unless verified there. The main crates remain `solar-parse`, `solar-ast`, `solar-sema`, `solar-interface`, `solar-config`, `solar-cli`, `solar-macros`, and `tools/tester`."
  key_issues: ["Fork baseline must be treated as the only owned implementation state.","Typeck remains the highest-leverage near-term track because it has the strongest built-in oracles.","Upstream codegen work (for example PR `#693`) is reference-only until explicitly adopted.","Parser/corpus tests do not prove runtime/codegen correctness.","The project is review-driven; correctness claims require evidence, not only green builds."]
  reference_docs: ["PADS.md","AGENTS.md","README.md","CONTRIBUTING.md",".github/workflows/ci.yml","tools/tester/src/lib.rs","crates/sema/src/typeck/mod.rs","benches/README.md"]
  notes: "Use `paradigmxyz/solar` as a reference source for roadmap shape, merged history, and useful design ideas. Do not assume upstream unmerged branches are already present here. For codegen/runtime work, Foundry is a downstream oracle layer, not the primary parser/sema oracle. The first job of the system is to understand the current fork baseline, not to fantasize about the future state."
---

# Solar Program Guidance

## Owned Baseline

The owned implementation baseline is the current `arjunblj/solar` default branch. That is the only state that counts for:

- bootstrap truth
- task generation
- progress tracking
- completion claims

Upstream `paradigmxyz/solar` is valuable, but only as a reference source:

- roadmap issues
- merged history
- open PRs
- docs and testing patterns

If an upstream PR is not merged into this fork, treat it as **reference-only**.

At the time this document was drafted, the audited fork baseline was commit `d79be54`.

## What Solar Is Trying To Become

Solar is a modular Rust Solidity compiler. The long-term target is not just “parse Solidity,” but to become a serious alternative to `solc` with:

- strong frontend correctness
- meaningful type-checking parity where Solar intends to match Solidity semantics
- high-quality diagnostics
- competitive performance
- MIR/codegen/runtime validation
- usable editor and tooling surfaces

This is a compiler program, not a feature app. The system should optimize for precise progress, not velocity theater.

## What Is Already Solid

- parser / lexer / AST fundamentals
- diagnostics infrastructure
- large parts of sema and typeck foundations
- override checker and many conversion rules
- UI test infrastructure
- benchmark infrastructure

## What Is Still Open

- broader `-Ztypeck` coverage and remaining semantic checks
- NatSpec lowering and similar frontend completeness work
- Yul / inline assembly lowering
- LSP milestones
- meaningful runtime/codegen takeover planning
- stronger differential and downstream execution infrastructure

## Definition Of Done By Track

### Typeck

Done means:

- the targeted semantic rule exists in the fork baseline
- it has focused UI tests
- it references the correct solc behavior or error code where appropriate
- corpus enablement changes measure exactly what new coverage is gained

### Diagnostics

Done means:

- output is correct and intentional
- `cargo uitest` passes
- snapshot changes are justified
- AGENTS.md style rules are preserved

### Codegen / Runtime

Done means:

- there is a named downstream oracle
- `solc` differential checks are defined
- Foundry/revm/anvil-style checks are specified or implemented
- “it compiles” is not treated as success

### Performance

Done means:

- correctness gates still pass
- benchmark claims are backed by actual numbers
- hot-path patterns remain explicit (`sym::`, `kw::`, arenas, visitor discipline)

### Yul / LSP

Done means:

- the feature is present in the fork baseline
- there are focused tests or conformance checks
- scope is small and reviewable

## Verification Model

### Frontend / Diagnostics / Sema

Primary ladder:

```bash
cargo build --workspace
cargo nextest run --workspace
cargo uitest
```

### Typeck

Primary ladder:

```bash
cargo nextest run --workspace
cargo uitest
TESTER_MODE=solc-solidity cargo test -p solar-compiler --test=tests
```

Use `solc 0.8.31` as the semantic reference for changed cases.

Important caveat: the current imported `solc` corpus runner is still primarily a parser/corpus oracle unless explicit `-Ztypeck` routing is enabled for the relevant semantic family. Do not over-claim what `TESTER_MODE=solc-solidity` proves on its own.

### Codegen / Runtime

Primary ladder:

- build/test/lint as above
- explicit `solc` differential checks
- Foundry downstream checks where artifacts are comparable
- revm/anvil-style execution checks for runtime behavior

Do not collapse this into a single “cargo test passed” story.

### Performance

Only run benchmarks when hot paths change:

```bash
cargo bench -p solar-bench --bench criterion -- --quick
```

Use the full benchmark suite only when needed.

## `solc` Relationship

`solc` is the reference semantics and corpus source where Solar intends to match Solidity behavior.

That means:

- use `solc` to validate specific changed cases
- use the Solidity corpus to understand coverage and gaps
- use exact error codes and behavior references where Solar already follows that convention

It does **not** mean:

- every green Solar test implies full `solc` parity
- parser-only corpus modes prove runtime correctness
- upstream `solc` expectations can be copied blindly without verifying the fork state

## Foundry’s Role

Foundry is useful, but not everywhere.

Use Foundry for:

- downstream project integration
- remapping-aware workflows
- ABI/runtime validation once codegen work exists in the fork
- extra confidence for runtime behavior

Do not use Foundry as a substitute for:

- UI diagnostics tests
- parser/sema correctness checks
- focused typeck verification

For many frontend and type-checking slices, Foundry is irrelevant and should not be pulled in just to make a task feel more “real.”

## Anti-Convergence Rules

The system must avoid shallow progress.

Every good task should name:

- the owned baseline it starts from
- the track it belongs to
- whether upstream is only reference material
- the exact file(s) likely involved
- the exact oracle that would prove success

Bad tasks:

- “investigate codegen”
- “deepen this finding”
- “improve Solar”

Good tasks:

- “From the current fork baseline, audit missing view/pure checks in `crates/sema/src/typeck/checker.rs` and add focused UI repros.”
- “Define the exact downstream oracle for adopting a codegen slice inspired by upstream PR `#693`, marking the upstream work as reference-only.”

## Review Culture

This project is review-driven.

Prefer:

- small, reviewable slices
- one logical change at a time
- precise tests
- explicit evidence

Avoid:

- giant speculative refactors
- claiming parity without an oracle
- updating snapshots without explanation
- assuming upstream branches are part of the fork
