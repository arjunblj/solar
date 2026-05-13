---
pads_version: 2
preset: compiler
spec_status: active
last_revised: "2026-05-12"
revision_trigger: evidence

tier0:
  project:
    slug: arjunblj/solar
    mission: >
      Make Solar a credible drop-in replacement for solc on the Solidity 0.8.x
      workflows real toolchains depend on: Standard JSON input and output,
      multi-file project compilation, structural diagnostics, ABI and metadata
      artifacts, source maps, storage layout, bytecode objects, and runtime
      behavior measurable against pinned solc builds. After correctness for a
      surface is measured, beat solc on it on real developer workflows. Use
      that correctness and performance shell as the safe substrate for
      speculative EVM compiler research.
    upstream:
      full_name: paradigmxyz/solar
      policy: reference_only
    baseline_commit: "9aad57d6956812b8b9b80a8d097d524fb6d5314d"
  hard_constraints:
    - Treat upstream paradigmxyz/solar as reference-only unless a task explicitly imports a source commit into this fork.
    - Never claim solc compatibility without naming solc version, EVM version, optimizer mode, corpus, and strongest passing oracle tier.
    - Never claim codegen or runtime correctness without bytecode or runtime equivalence evidence, or explicitly label the PR infrastructure-only.
    - Never claim performance without a correctness gate, same-environment baseline, benchmark command, before/after numbers, and profile evidence.
    - Never edit or rebless UI snapshots without paired source changes and a reviewed semantic before/after.
    - Never grow skip or xfail lists without issue link, reason, owner track, and revisit condition.
    - Never commit corpora caches, generated traces, sandbox artifacts, out/, cache/, benchmark images, or vendored Foundry dependencies.
    - Never bump MSRV, change release/publish config, add top-level workspace dependencies, or make broad dependency updates without human approval.
    - Never merge feat/codegen-mir wholesale.
  scope_of_autonomy:
    permitted_subgoals:
      - close measurable deltas against solc behavior
      - build and improve compiler oracles and corpora
      - land large coherent compiler PRs on this fork
      - extract reviewable slices from upstream branches with attribution
      - improve performance only with correctness-preserving evidence
      - run speculative compiler experiments behind isolated flags or paths
      - update PADS.md and .pads files when project context improves
    permitted_side_effects:
      max_files_per_pr: 80
      max_crates_per_pr: 8
      max_wall_time_per_task_min: 480
      max_api_spend_per_task_usd: 100
    must_pause_for_approval:
      - .github/workflows/**
      - deny.toml
      - clippy.toml
      - rustfmt.toml
      - rust-toolchain.toml
      - testdata/solidity/**
    shutdown_timer:
      mode: disabled
    high_risk_prohibitions:
      - broad dependency updates
      - MSRV bumps
      - release or crate publishing changes
      - wholesale codegen branch merge
      - production compiler rewrites without oracle plan
      - optimizer or runtime claims without runtime or differential evidence
  edit_rules:
    - PADS.md is the stable project constitution; .pads/spec.json is the generated mirror.
    - Tier-0 edits require updating PADS.md, .pads/spec.json, and .pads/tier0.sha256 together.
    - .pads/tier0.sha256 may be updated without human approval only as the checksum output of scripts/pads/tier0-guard.py after paired PADS.md and .pads/spec.json edits; policy-changing Tier-0 edits still need explicit rationale in the PR.
    - Project-specific Solar, solc, Foundry, corpus, and maintainer context belongs here or under .pads, never in pads core.
    - Cargo manifest edits are permitted when they are required for a reviewable compiler or harness slice; Cargo.lock changes must be minimal and explained, and dependency/MSRV/release/publish risk still requires human approval.
    - New durable campaign facts should first go to wiki, memory, tracking issues, or follow-up PRs; update PADS.md when the fact becomes stable project policy.
    - This file is policy, context, and a senior-engineer briefing. It is not a backlog template. Generated GitHub issues should be substantial components of work that yield meaningful PRs, not a mechanical decomposition of this file's headings, tracks, oracles, or tier numbers.

campaign_state:
  epoch: "2026-05-13"
  fork_main_commit: "8d26b642fe195b4594d8509e07120cce70a80149"
  upstream_main_commit: "0573b99c26c4ed5ff951ecd8e16e11f652fdaff8"
  upstream_codegen_mir_commit: "69d2521c02d5d4ca63c8ba2598b2d67bdf099280"
  upstream_runtime_equivalence_pr: "paradigmxyz/solar#760"
  organizer_completion_brief: "2026-05-13 Solar completion context after upstream main refresh and Pads quality-loop audit"
  refresh_before_dispatch: false
  refresh_items:
    - fork compare against upstream main only when planner evidence cites stale upstream state
    - fork open PRs and unsafe dependency updates
    - upstream issue and PR watchlist state
    - upstream branch state, especially feat/codegen-mir and active typeck branches
    - gakonst codegen roadmap and runtime-equivalence comments
    - latest CI status and required versus advisory failures
    - corpus counts and skip/xfail deltas
    - whether generated plan/issues/prompts use the organizer completion brief to produce repo-grounded work

sandbox_profile:
  required_bins: [cargo, rustc, cargo-nextest, typos, python3]
  oracle_bins: [solc, solc-select, forge]
  optional_bins: [jq, uv, anvil, cargo-hack, cargo-codspeed, cargo-docs-rs, bun, node, pnpm, gh]
  env:
    SOLC_VERSION: "0.8.31"
    CARGO_BUILD_JOBS: "1"
  bootstrap_hooks:
    - bash .pads/setup.sh
  package_managers: [cargo, foundry, uv, pnpm]
  warmup_commands:
    - git submodule update --init --checkout testdata/solidity
    - initialize nested Solidity submodules only when a selected corpus or upstream build requires them
    - cargo fetch --locked
    - solc --version if available; otherwise mark solc differential lanes unavailable
    - forge --version if available; otherwise mark Foundry replay lanes unavailable
    - cargo nextest --version
  cache_paths:
    - .cargo
    - target
    - out
    - cache
  resource_hints:
    timeout_minutes: 240
    max_nested_shard_concurrency: 6

pr_queue_policy:
  max_open_autonomous_drafts: 3
  max_open_prs_touching_same_primary_file: 1
  before_opening_pr:
    - search fork issues and pull requests for overlapping work
    - close or supersede obsolete generated issues before fresh ingest
    - reject zero-diff pull requests before GitHub publication
    - reject .pads-artifacts, sandbox logs, raw local traces, and workspace artifacts in diffs
    - include at least one focused oracle result or an explicit blocker in the PR body
  ready_for_review_requires:
    - local required gates or focused task oracles pass with visible evidence
    - an independent reviewer agent judges the diff coherent and mergeable in substance
    - GitHub CI failures are classified as required, advisory, flaky, or baseline-noisy
    - known skipped or deferred checks are justified in the PR body

artifact_hygiene:
  readonly_globs:
    - testdata/solidity/**
    - .github/workflows/**
    - rust-toolchain.toml
    - deny.toml
    - clippy.toml
    - rustfmt.toml
  snapshot_globs:
    - tests/ui/**/*.stderr
    - tests/ui/**/*.stdout
  snapshot_suffixes: [".stderr", ".stdout", ".snap"]

branch_policy:
  default_upstream_mode: reference_only
  write_target: fork_only
  require_import_decision: true
  watchlist:
    - id: roadmap
      source: paradigmxyz/solar
      upstream_ref: issues/1
      mode: track
      priority: critical
      related: ["paradigmxyz/solar#1"]
      notes: "Primary public roadmap: Standard JSON, typeck, Yul/HIR, ABI parity, MIR/backend, runtime tests."
    - id: typeck
      source: paradigmxyz/solar
      upstream_ref: issues/615
      mode: track
      priority: critical
      related: ["paradigmxyz/solar#615"]
      notes: "Typechecker policy: every typeck change needs tests and solc 0.8.31 comparison."
    - id: solc-typeerror-corpus
      source: paradigmxyz/solar
      upstream_ref: pull/737/head
      mode: port
      priority: high
      related: ["paradigmxyz/solar#663", "paradigmxyz/solar#737"]
      notes: "Reference for exposing solc TypeError corpus under -Ztypeck."
    - id: codegen-roadmap
      source: paradigmxyz/solar
      upstream_ref: issues/687
      mode: track
      priority: critical
      related: ["paradigmxyz/solar#687", "paradigmxyz/solar#694", "paradigmxyz/solar#704"]
      notes: "Dependency graph for MIR/codegen and runtime equivalence."
    - id: codegen-mir-draft
      source: paradigmxyz/solar
      upstream_ref: pull/693/head
      mode: extract
      priority: critical
      related: ["paradigmxyz/solar#693", "paradigmxyz/solar#749", "paradigmxyz/solar#756", "paradigmxyz/solar#760"]
      notes: "Open draft with broad codegen work; use as source material, never blind merge. Current branch tip observed 2026-05-10: 69d2521."
    - id: codegen-runtime-equivalence
      source: paradigmxyz/solar
      upstream_ref: pull/760/head
      mode: extract
      priority: critical
      related: ["paradigmxyz/solar#760", "paradigmxyz/solar#704", "paradigmxyz/solar#693"]
      notes: "Solar-vs-solc codegen CI is currently red on runtime mismatches. Mine the comparator/reporting shape as red/xfail infrastructure; do not claim codegen correctness from it."
    - id: yul-hir
      source: paradigmxyz/solar
      upstream_ref: issues/415
      mode: track
      priority: high
      related: ["paradigmxyz/solar#415", "paradigmxyz/solar#652"]
      notes: "Design-risk area: Yul semantics can diverge from Solidity HIR assumptions."
    - id: frontend-performance
      source: paradigmxyz/solar
      upstream_ref: pull/754/head
      mode: track
      priority: high
      related: ["paradigmxyz/solar#475", "paradigmxyz/solar#508", "paradigmxyz/solar#754"]
      notes: "Performance reference; require correctness gates before claiming wins."
    - id: small-upstream-fixes
      source: paradigmxyz/solar
      upstream_ref: open-prs
      mode: cherry_pick
      priority: high
      related: ["paradigmxyz/solar#743", "paradigmxyz/solar#744", "paradigmxyz/solar#755", "paradigmxyz/solar#758", "paradigmxyz/solar#761"]
      notes: "Narrow parser/diagnostic/sema fixes; verify freshness and tests before importing. PR #761 is already included in fork main as 0573b99/8d26b64, so do not generate it as new work."
    - id: divergence-caution
      source: paradigmxyz/solar
      upstream_ref: issues/547
      mode: track
      priority: medium
      related: ["paradigmxyz/solar#547", "paradigmxyz/solar#689"]
      notes: "Do not document solc divergence without maintainer guidance."

tracks:
  - id: compatibility-matrix
    name: Compatibility Matrix
    priority: critical
    status: active
    scope: ["crates/**", "tools/**", "testdata/**", "benches/**"]
    archetypes: [sota-experiment, compiler-harness]
    required_oracles: [cargo.nextest, solc.standard_json.frontend]
  - id: standard-json
    name: Standard JSON and Tool Interface
    priority: critical
    status: active
    scope: ["crates/interface/**", "crates/solar/**", "tools/**", "testdata/**"]
    required_oracles: [solc.standard_json.frontend]
  - id: parser-ast-diagnostics
    name: Parser, AST, Diagnostics
    priority: high
    status: active
    scope: ["crates/parse/**", "crates/ast/**", "crates/interface/src/diagnostics/**", "tests/ui/**"]
    required_oracles: [cargo.uitest, solc.syntax]
  - id: typeck-corpus
    name: Typechecker and solc Corpus
    priority: critical
    status: active
    scope: ["crates/sema/**", "crates/solar/**", "tools/tester/**", "tests/ui/**"]
    upstream_tracking: ["paradigmxyz/solar#615", "paradigmxyz/solar#663", "paradigmxyz/solar#737"]
    required_oracles: [cargo.uitest, solc.typeck]
  - id: abi-natspec-sourcemaps
    name: ABI, NatSpec, Metadata, Source Maps
    priority: high
    status: active
    scope: ["crates/interface/**", "crates/sema/**", "crates/solar/**", "tests/ui/**"]
    required_oracles: [solc.standard_json.frontend]
  - id: yul-hir-boundary
    name: Yul and HIR Boundary
    priority: high
    status: active
    scope: ["crates/parse/**", "crates/sema/**", "crates/ast/**", "tests/ui/**"]
    upstream_tracking: ["paradigmxyz/solar#415", "paradigmxyz/solar#652"]
    required_oracles: [solc.yul]
  - id: mir-codegen
    name: MIR and Codegen Critical Path
    priority: critical
    status: draft
    scope: ["crates/mir/**", "crates/codegen/**", "crates/solar/**", "tests/ui/**"]
    upstream_tracking: ["paradigmxyz/solar#687", "paradigmxyz/solar#693", "paradigmxyz/solar#749"]
    required_oracles: [mir.roundtrip, solc.bytecode, runtime.equivalence]
  - id: runtime-equivalence
    name: Bytecode and Runtime Equivalence
    priority: critical
    status: draft
    scope: ["crates/**", "tools/**", "testdata/**"]
    upstream_tracking: ["paradigmxyz/solar#704"]
    required_oracles: [runtime.equivalence]
  - id: foundry-hardhat
    name: Foundry and Hardhat Integration
    priority: critical
    status: active
    scope: ["crates/interface/**", "crates/solar/**", "tools/**", "scripts/**"]
    required_oracles: [foundry.config, foundry.standard_json]
  - id: performance
    name: Performance
    priority: medium
    status: active
    scope: ["benches/**", "crates/parse/**", "crates/sema/**", "crates/interface/**", "crates/solar/**"]
    required_oracles: [perf.codspeed, perf.iai]
  - id: fuzz-metamorphic
    name: Fuzz and Metamorphic Testing
    priority: medium
    status: active
    scope: ["tools/**", "testdata/**", "crates/**", "fuzz/**"]
    required_oracles: [fuzz.differential]
  - id: speculative-research
    name: Speculative Compiler Research
    priority: research
    status: active
    scope: ["research/**", "crates/**", "tools/**"]
    required_oracles: [research.artifact]

priority_order:
  - compatibility-matrix
  - standard-json
  - typeck-corpus
  - foundry-hardhat
  - runtime-equivalence
  - mir-codegen
  - parser-ast-diagnostics
  - abi-natspec-sourcemaps
  - yul-hir-boundary
  - performance
  - fuzz-metamorphic
  - speculative-research

north_star_components:
  - title: Standard JSON process contract
    outcome: solar --standard-json behaves like the compiler syscall real Solidity tools expect, including stdin/stdout shape, settings subset, output selection, and structurally compatible errors.
  - title: Project import and source identity semantics
    outcome: source-unit names, base/include/allow paths, remappings, and metadata-sensitive paths line up with solc for real multi-file projects so downstream tools key contracts and errors the same way.
  - title: Structural diagnostics and typechecker corpus
    outcome: parser, AST, diagnostics, and typechecking are measured against pinned solc on owned fixtures and corpus slices with stable counters and explicit xfail accounting.
  - title: Artifact emission surface
    outcome: ABI, NatSpec userdoc/devdoc, metadata (string and CBOR), storage layout, method identifiers, and source maps are produced through the Standard JSON path downstream tools consume.
  - title: Executable compiler spine
    outcome: HIR or sema state lowers through reviewable IR slices into an EVM assembler, bytecode objects, link references, immutable references, and metadata, with normalized differential evidence against solc.
  - title: Runtime equivalence
    outcome: Solar-built bytecode executes against pinned solc bytecode on a runtime corpus comparing returns, reverts, logs, storage, and gas class.
  - title: Foundry and Hardhat replay
    outcome: real project compiler inputs (foundry.toml, remappings, build-info, OpenZeppelin/Solady/Solmate/Uniswap/Seaport-class corpora) replay through Solar with declared output parity or explicit unsupported ledgers.
  - title: Correctness-gated speed
    outcome: Solar gets faster only on surfaces whose correctness oracle already passes, with paired same-environment baselines, profiles, and before/after evidence.

planning_guidance:
  - Treat PADS.md and AGENTS.md as organizer-supplied kickoff context that the autonomous harness should ingest and rely on when planning. For implementation claims, PR bodies, and completion claims, pair that context with concrete repo code, upstream issue/PR/branch evidence, oracle output, or diffs.
  - The operator is not manually finishing Solar. The product goal is that the Pads harness can ingest this file plus the repo and then autonomously behave like a senior compiler team over weeks, discovering, planning, implementing, verifying, reviewing, publishing, learning from upstream, and replenishing work without a hand-written mission.
  - The upstream vision is the starting map, especially gakonst's codegen/MIR roadmap and runtime-equivalence comments. The autonomous campaign should first reach baseline correctness and observability, then surpass upstream performance and research quality on this fork with explicit oracles.
  - The initial generated GitHub issue list should be substantial components-of-work, each one a meaningful PR a senior engineer would land. The master issue should explain how those components fit together end-to-end, link them, and describe what shipped looks like. The master issue is not itself a PR.
  - Every issue should describe what the change is, the engineering approach, how to know it is done (testing/checks), and any related issues. Do not include time estimates, deadlines, line-of-code budgets, file-count targets, or scope-class enums in the issue body. Size work by the behavior it produces and the proof it requires.
  - Do not generate research-question issues ("What is...?", "Which...?", "Investigate...") when a senior engineer can infer the next engineering move from this charter, the repository, the reference compiler, and the needs of downstream toolchains. If a real decision is genuinely blocking, capture it as planner research, not as a public issue.
  - Do not decompose one coherent component into many small chore issues. Group the production change, the fixtures, and the verifier together when they prove one invariant. A component that needs a new harness, new fixtures, and a production change is one issue.
  - Do not parrot this file's track names or oracle ladder tier numbers into the issue list. Tracks and oracles are project policy and verification grammar. Issues are concrete components of compiler work.
  - Durable repository tooling should fit the repository's native engineering system. Solar is a Rust compiler; default to Rust-native tooling or existing Rust test/harness surfaces for project-owned durable tools. Python or shell is fine for tiny glue only when it follows an existing convention or avoids committing generated/cache artifacts. If native tooling requires locked manifest edits, surface the approval blocker rather than silently switching languages.
  - The track ladder, watchlist, and oracle definitions exist so generated work cannot overclaim. They do not define what to ship. The plan should reflect what a senior engineer would actually do to move Solar toward the completion contract; the policy here ensures the proof boundary stays honest.
  - Penalize generated plans, issues, and worker prompts if they equate draft codegen with compiler completion, ignore pinned solc 0.8.31, count ignored exits as passes, skip Standard JSON or Yul, optimize before correctness oracles, import generated Foundry caches, or use broad parity language without exact corpus/version/settings/field evidence.
  - Do not create a "fix typeck corpus slice" issue that uses `TESTER_MODE=solc-solidity` as proof. That mode is parser-oriented today. The first typechecker PR train must create or tighten the `-Ztypeck`/TypeError measurement lane, fixture selection, xfail accounting, and reporting contract before claiming typechecker corpus fixes.
  - Standard JSON work starts with the current CLI/config/interface/sema entry points and must name the exact supported input fields, unsupported-field diagnostic shape, and compared output fields. If `solar --standard-json` is not implemented in the checked-out code, the first PR is a front-door/subset contract PR, not an artifact-parity claim.
  - Runtime and bytecode work starts as comparison infrastructure with explicit unsupported results. Do not route runtime-equivalence workers into codegen correctness claims until Solar can emit bytecode for the selected subset and the comparator has pinned solc provenance.
  - A high-quality kickoff plan starts with measurement and correctness, then branch-mines upstream into a dependency-ordered branch train. It should not parrot this file's phase names or create research-question issues when the next engineering move is inferable.

oracles:
  - { id: cargo.fmt, kind: shell, tier: advisory, time_budget_s: 120, command: "cargo +nightly fmt --all --check" }
  - { id: typos, kind: shell, tier: prerequisite, time_budget_s: 120, command: "typos --format brief" }
  - { id: cargo.check, kind: shell, tier: prerequisite, time_budget_s: 900, command: "cargo check --workspace" }
  - { id: cargo.build, kind: shell, tier: prerequisite, time_budget_s: 1200, command: "cargo build --workspace" }
  - { id: cargo.clippy, kind: shell, tier: gate, time_budget_s: 1200, command: "cargo clippy --workspace --all-targets -- -D warnings" }
  - { id: cargo.nextest, kind: shell, tier: gate, time_budget_s: 1800, command: "cargo nextest run --workspace" }
  - { id: cargo.doctest, kind: shell, tier: gate, time_budget_s: 900, command: "cargo test --doc --workspace" }
  - { id: cargo.uitest, kind: shell, tier: gate, time_budget_s: 1200, command: "cargo uitest" }
  - { id: solc.syntax, kind: shell, tier: gate, time_budget_s: 1800, command: "TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests", corpus_ref: solc-syntax-tests }
  - { id: solc.yul, kind: shell, tier: gate, time_budget_s: 1800, command: "TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests", corpus_ref: solc-yul-tests }
  - id: solc.typeck
    kind: shell
    tier: advisory
    corpus_ref: solc-typeerror-tests
    status: blocked_until_typeerror_lane_exists
    notes: "Current TESTER_MODE=solc-solidity is parser-oriented and ignores TypeError exits; a real typeck oracle must pass -Ztypeck, filter TypeError fixtures, and report xfail deltas."
  - id: solc.standard_json.frontend
    kind: semantic_differential
    tier: advisory
    status: desired_until_standard_json_front_door_exists
    corpus_ref: standard-json-smoke
    reference: { compiler: solc, interface: standard-json, version: "0.8.31" }
    under_test: { compiler: solar, interface: standard-json }
    compare: [errors, sources, contracts, abi, userdoc, devdoc, storageLayout, methodIdentifiers]
  - id: foundry.config
    kind: shell
    tier: advisory
    time_budget_s: 120
    status: blocked_until_foundry_fixture_selected
    command: "cd $PADS_FOUNDRY_PROJECT && test -f foundry.toml && forge config --json && forge remappings"
    notes: "Do not count forge's default repo-root config as Foundry support; select a real fixture or project path first."
  - id: foundry.standard_json
    kind: semantic_differential
    tier: advisory
    status: desired_until_replay_tool_exists
    corpus_ref: foundry-build-info
    reference: { compiler: solc, interface: standard-json }
    under_test: { compiler: solar, interface: standard-json }
    compare: [errors, abi, metadata, storageLayout, build-info]
  - id: mir.roundtrip
    kind: shell
    tier: advisory
    time_budget_s: 900
    status: blocked_until_mir_crate_exists
    command: "cargo test -p solar-mir"
    notes: "Current main has no solar-mir package, crates/mir, crates/codegen, or solar-mir-opt; these are future target paths for the codegen branch train."
  - id: solc.bytecode
    kind: differential
    tier: advisory
    status: desired_until_codegen_outputs_exist
    corpus_ref: runtime-micro
    reference: { compiler: solc, normalize: metadata-stripped-bytecode }
    under_test: { compiler: solar, normalize: metadata-stripped-bytecode }
    compare: [creation_bytecode, runtime_bytecode, link_refs, immutable_refs]
  - id: runtime.equivalence
    kind: hevm_equivalence
    tier: advisory
    status: desired_until_runtime_harness_exists
    corpus_ref: runtime-micro
    compare: [return_data, revert_data, logs, storage, gas]
  - id: fuzz.differential
    kind: metamorphic
    tier: advisory
    corpus_ref: fuzz-minimized
    compare: [accept_reject, runtime_equivalence, diagnostic_category]
  - { id: perf.codspeed, kind: shell, tier: advisory, time_budget_s: 2400, command: "cargo codspeed build && cargo codspeed run" }
  - { id: perf.iai, kind: shell, tier: advisory, time_budget_s: 1800, command: "cargo bench -p solar-bench --bench iai" }
  - { id: research.artifact, kind: shell, tier: advisory, time_budget_s: 30, command: "test -d research || true" }

corpora:
  - id: solar-native
    source: arjunblj/solar:testdata
    commit: "9aad57d6956812b8b9b80a8d097d524fb6d5314d"
    phase: fast-smoke
    setup:
      - "git submodule update --init --checkout testdata/solidity"
      - "do not recurse into Solidity deps unless the selected corpus/build requires them"
    proves: ["Solar-owned UI, parser, diagnostics, and import behavior"]
    does_not_prove: ["Full solc compatibility", "runtime equivalence"]
  - id: solc-syntax-tests
    source: https://github.com/argotorg/solidity/tree/develop/test/libsolidity/syntaxTests
    commit: pin-after-first-fetch
    filter_path_glob: test/libsolidity/syntaxTests/**/*.sol
    phase: parser-typeck
    proves: ["parser accept/reject behavior", "diagnostic category pressure"]
    does_not_prove: ["runtime correctness", "bytecode equivalence"]
  - id: solc-typeerror-tests
    source: https://github.com/argotorg/solidity/tree/develop/test/libsolidity/syntaxTests
    commit: pin-after-first-fetch
    filter_path_glob: test/libsolidity/syntaxTests/**/*TypeError*.sol
    phase: typeck
    proves: ["type checker pressure against solc 0.8.x"]
    does_not_prove: ["complete semantic equivalence"]
  - id: solc-yul-tests
    source: https://github.com/argotorg/solidity/tree/develop/test/libyul
    commit: pin-after-first-fetch
    phase: yul
    proves: ["Yul parse/optimizer/interpreter coverage"]
    does_not_prove: ["Solidity HIR compatibility"]
  - id: standard-json-smoke
    source: docs.soliditylang.org standard-json examples plus reduced Solar fixtures
    commit: repo-owned
    phase: standard-json
    setup:
      - "solc --standard-json < input.json"
      - "future: solar --standard-json < input.json after the Solar front door exists"
    proves: ["declared Standard JSON field parity"]
    does_not_prove: ["undeclared output fields", "runtime behavior"]
  - id: foundry-build-info
    source: foundry-rs/forge-std, OpenZeppelin, Solady, Solmate, PRBMath, Uniswap
    commit: pin-per-project
    phase: foundry
    setup: ["forge build --force --build-info --build-info-path out/build-info"]
    proves: ["Foundry project config, remappings, build-info capture"]
    does_not_prove: ["Solar runtime correctness until Solar replays the inputs"]
  - id: runtime-micro
    source: repo-owned reduced contracts with scripted calls
    commit: repo-owned
    phase: runtime
    proves: ["runtime behavior for covered deploy/call/revert/log/storage cases"]
    does_not_prove: ["complete deployed behavior"]
  - id: fuzz-minimized
    source: OSS-Fuzz, solc-fuzz, Soltix, Foundry/Echidna/Medusa/Halmos/hevm findings
    commit: pin-per-import
    phase: fuzz
    proves: ["reduced differential and crash regressions"]
    does_not_prove: ["absence of bugs"]
  - id: performance-corpus
    source: Solar benches plus pinned ecosystem projects
    commit: pin-per-project
    phase: performance
    setup: ["cargo codspeed build && cargo codspeed run", "cargo bench -p solar-bench --bench iai"]
    proves: ["measured compiler performance on declared workloads"]
    does_not_prove: ["correctness without paired correctness oracle"]

anti_rabbit_hole:
  refuse_on_name:
    - "merge feat/codegen-mir"
    - "full rewrite"
    - "bump all dependencies"
    - "fix everything"
    - "cargo uibless"
  step_back_triggers:
    - "A task tries to claim codegen or runtime parity without T7/T8 evidence."
    - "A task grows skip/xfail lists instead of exposing the mismatch."
    - "A performance task lacks a correctness oracle and baseline."
    - "A branch import lacks source commit list and omitted-work explanation."
  budgets:
    max_wall_minutes_per_task: 480
    max_cost_usd_per_task: 100
    max_retries_per_symptom: 3

# starter_tasks block intentionally removed (2026-05-09).
# Earlier versions of PADS shipped 5 calibration examples here. The Pads harness
# was echoing them back as the entire kickoff TASK_GRAPH, biasing every run
# toward green-field scaffolding even though Solar already ships parser, sema,
# interface, tooling, UI fixtures, and bench infra. The planner now reads the
# completion_contract below, the ## Phase Model section, oracles, tracks,
# upstream context, and current GitHub/CI state, then computes the next
# delta-from-done slices for itself. Do not reintroduce starter_tasks unless
# you have evidence that the planner cannot find first work without them.

completion_contract:
  - "Solar supports the declared Solidity 0.8.x compatibility matrix or records every remaining gap with owner, oracle, corpus, and blocker."
  - "Standard JSON is the default integration surface for supported outputs."
  - "Foundry build-info replay works for the pinned framework corpus on supported outputs."
  - "Typeck, diagnostics, ABI, NatSpec, metadata, and source-map parity are measured against pinned solc versions."
  - "MIR/codegen critical path has bytecode and runtime equivalence or explicit unsupported ledgers."
  - "Performance claims have correctness gates, baselines, profiles, and before/after numbers."
  - "Speculative research tracks produce fixtures, minimized counterexamples, or gated experiments without polluting production paths."

extensions:
  solar:
    current_snapshot:
      fork_main_commit: "8d26b642fe195b4594d8509e07120cce70a80149"
      upstream_main_commit: "0573b99c26c4ed5ff951ecd8e16e11f652fdaff8"
      compare_url: "https://github.com/paradigmxyz/solar/compare/main...arjunblj:main"
      upstream_refresh_note: "Fork main includes upstream PR #761 bare integer alias canonicalization. Treat further upstream refresh as event-driven context repair, not a required pre-dispatch ritual."
    unsafe_fork_prs:
      - "arjunblj/solar#556: weekly Cargo.lock update pulls crates requiring Rust 1.90/1.91.1 while rust-toolchain.toml pins 1.88.0."
    ci_baseline:
      passing: [stable OS tests, fmt, clippy, typos, deny]
      needs_attention: [features, docs, test ubuntu-latest nightly]
      advisory_failures: [CodSpeed auth 401]
    maintainer_principles:
      - "Correctness is the currency."
      - "Make progress little by little to keep upstream code quality high."
      - "Foundry integration and mixed solc/Solar runtime testing are first-class."
      - "Performance only matters with correctness and benchmark evidence."
    current_state:
      shipped:
        - lexer/parser, AST, diagnostics, file resolver, HIR lowering, sema/typeck WIP, ABI/hash emission, UI tests, solc syntax and Yul corpus runners
        - upstream bare integer alias canonicalization from paradigmxyz/solar#761 is present in this fork
      not_shipped:
        - production Standard JSON front door
        - real TypeError/-Ztypeck corpus oracle with non-ignored exits
        - Foundry build-info replay through Solar
        - bytecode/codegen/runtime equivalence on main
      current_default_principle: "Prefer measured gaps and proof-producing tooling over compatibility claims."
    known_gaps:
      - id: oracle_inventory_missing
        track: compatibility-matrix
        current_evidence: "PADS and AGENTS define commands, but no deterministic repo-owned report yet records tool versions, corpus counts, skip counts, and supported oracle tiers."
        next_measurement: "Create a small repo-native oracle inventory report before dispatching broad compatibility fixes."
        proof_boundary: "Measurement artifact only; does not claim compatibility improvement."
      - id: standard_json_frontdoor_missing
        track: standard-json
        current_evidence: "Current main exposes combined-json style ABI/hash output, but no production Standard JSON stdin/stdout front door."
        next_measurement: "Identify CLI/config/sema entry points and supported minimal fields before artifact parity work."
        proof_boundary: "A first PR may claim only supported input fields and honest unsupported-field diagnostics."
      - id: typeerror_lane_missing
        track: typeck-corpus
        current_evidence: "TESTER_MODE=solc-solidity is parser-oriented; it is not a TypeError/typeck parity oracle."
        next_measurement: "Add or tighten a distinct TypeError/-Ztypeck lane with pass/fail/unsupported/xfail counters."
        proof_boundary: "Do not claim typechecker corpus movement until the lane names fixture IDs and exit semantics."
      - id: foundry_build_info_capture_missing
        track: foundry-hardhat
        current_evidence: "Foundry oracles are advisory until a real fixture/project path is selected."
        next_measurement: "Capture forge config/remappings/build-info for a tiny pinned fixture without claiming Solar replay support."
        proof_boundary: "Input capture only; no runtime or build correctness claim."
      - id: bytecode_equivalence_future
        track: runtime-equivalence
        current_evidence: "Main lacks production codegen/bytecode output; upstream codegen branches are source material only."
        next_measurement: "Build comparator/mismatch artifact schema or mine upstream branch slices after prerequisites exist."
        proof_boundary: "Infrastructure-only unless Solar can emit bytecode for selected subset."
    starter_calibration_slices:
      - title: Oracle inventory and baseline ledger
        mode: calibration
        unlocks: "All later measured tasks cite current tool/corpus/support state instead of guessing."
        proof: "deterministic JSON or markdown report with tool versions, corpus counts, skip/xfail counts, and unavailable oracles"
      - title: Minimal Standard JSON front door
        mode: calibration
        unlocks: "Diagnostics, ABI, metadata, storage layout, and project replay can share the same integration surface."
        proof: "stdin JSON parse plus supported language/sources/settings subset and unsupported-field diagnostics"
      - title: TypeError measurement lane
        mode: calibration
        unlocks: "Typeck implementation tasks can target named fixture families instead of broad corpus themes."
        proof: "TESTER_MODE or equivalent lane with -Ztypeck semantics, counters, xfail schema, and fixture IDs"
      - title: Foundry build-info capture
        mode: calibration
        unlocks: "Framework replay tasks can consume real Standard JSON inputs without claiming runtime support."
        proof: "forge config/remappings/build-info captured from a pinned tiny fixture"
    continuation_rules:
      - "After a measurement artifact lands, choose implementation work from the largest newly measured failing family with owner files and a cheapest oracle."
      - "After a Standard JSON front-door PR lands, advance diagnostics/source identity before artifact-output parity."
      - "After a review rejects a patch for missing fixture IDs or proof, create a measurement task unless the fixture family is already known."
      - "After upstream moves in an owned lane, refresh only that lane's evidence before dispatching related work."
      - "Do not replay calibration slices once they have produced their unlock artifact; continue from live evidence."
    pr_proof_rules:
      - "Semantic PRs need at least one non-format proof or an explicit typed reason why the task is measurement-only."
      - "Formatter evidence is a checkpoint, not a compatibility proof."
      - "Corpus PRs must name fixture IDs, corpus path family, solc version, and pass/fail/unsupported delta."
      - "Standard JSON PRs must name supported fields and unsupported outputs."
      - "Branch-mining PRs must name upstream commit(s), omitted work, conflicts, and local proof boundary."

non_goals:
  - SMTChecker parity unless separately approved.
  - Legacy Solidity language modes outside the declared 0.8.x compatibility surface.
  - Documentation-only churn that does not unlock implementation or verification.
  - Bytecode, runtime, optimizer, or performance claims ahead of the oracles required to prove them.
---
# Solar Autonomous Compiler Campaign

This file is the kickoff brief and operating constitution for autonomous work on `arjunblj/solar`. It is intentionally long. It is policy, project context, and the senior-engineer briefing a strong team would want before picking up Solar cold.

It is not a backlog. The structured sections above (`tracks`, `oracles`, `corpora`, `watchlist`, `extensions`) are project policy and verification grammar. They keep generated work honest. They do not enumerate which PRs to open. The `north_star_components` and `planning_guidance` blocks describe the shape of the work that should be planned. Everything below is project context a planner reads to write a real engineering plan.

The run should assume this mission when no explicit user prompt is supplied. The orchestrator should continuously discover, localize, rank, implement, review, publish, merge, and replenish work until the original mission is complete or remaining work is low value.

## Kickoff Operating Instructions

The first no-mission kickoff should not wait for a human to enumerate work. The mission is this file.

1. Bootstrap with `.pads/setup.sh` and record the exact tool versions.
2. Read this `PADS.md`, `AGENTS.md`, `.pads/spec.json`, the fork diff, upstream issue/PR watchlist, and current CI status.
3. Refresh `campaign_state` in memory/wiki/tracking artifacts before dispatching code work: fork head, upstream head, open fork PRs, unsafe dependency updates, failing checks, and corpus counts.
4. Plan the work the way a staff engineer would. Read the repo, this charter, the reference compiler, and the needs of downstream toolchains, then write a real engineering plan: substantial components-of-work that move Solar toward the completion contract.
5. Project issues are memory, not the work itself. Create the smallest useful public issue surface for the current plan: usually one campaign control issue plus linked component issues once the branch train and proof boundary are clear. Do not block implementation on perfect issue hierarchy, and do not create phase-heading issues when a worker can already start a reviewable compiler slice.
6. Do not look for `starter_tasks`; they were intentionally removed. Synthesize the real plan from repo evidence, upstream state, CI/corpus reality, prior worker handoffs, and the completion contract.
7. Do not seed issues from this file's headings, tracks, oracles, or tiers. Do not generate "Investigate ..." or "What ...?" issues when the next engineering move is inferable. Do not anchor work to dates, durations, file counts, or line counts. Size work by the behavior and proof it needs.
8. Start with the highest-leverage correctness and measurement work, not with isolated parity chores. Standard JSON I/O, project import semantics, structural diagnostics, artifact emission through Standard JSON, and the executable compiler spine are connected; group changes so a single PR proves an invariant a maintainer can review.
9. Do not dispatch codegen, runtime, optimizer, or performance claims unless the work states the proof tier it can support and the missing dependencies.
10. Preserve context after every completed or blocked worker: what changed, what failed, decisive evidence, next dependency, and the track it belongs to.
11. Keep running until the orchestrator can explain that the original mission is complete or remaining work is low value.

Run-state policy:

Use clean resets only while validating fresh ingestion, schema compatibility, or bootstrap behavior. Once a run has a coherent plan, useful frontier state, worker evidence, open draft PRs, or unresolved but valuable blockers, prefer resume, repair, replan, or targeted cleanup over full reset. The product goal is cumulative long-running autonomy, not discarding useful team memory after every harness improvement.

## North Star

Solar should become the Rust-native Solidity compiler that can replace `solc` for Solidity 0.8.x, beat `solc` on real developer workflows, and become a safe research platform for EVM compiler ideas after correctness and performance are measurable.

This campaign has three compounding arcs:

1. **Baseline correctness:** establish the compatibility matrix, corpus program, Standard JSON front door, typechecker corpus, diagnostics/ABI/NatSpec/source-map parity, and Foundry/Hardhat input compatibility.
2. **Performance:** once a compiler stage has a correctness oracle, make Solar faster on that stage with same-environment baselines, profiles, and real corpora.
3. **Speculative compiler research:** behind safe flags or isolated paths, explore better IR, optimization, formal methods, fuzzing, incremental compilation, LSP feedback, EVM/gas analysis, EOF/new opcode support, and rewrite-rule synthesis.

The goal is not "open some PRs." The goal is to build a machine that continuously closes measured deltas against `solc`, proves the closures, and produces upstream-quality PR stacks.

## Maintainer Ethos

Act like an ambitious Solar maintainer, not a generic bugfix bot.

Durable principles distilled from upstream Solar, Foundry, and Paradigm guidance:

- Correctness is the currency.
- Make progress little by little, but the slices can be large when the invariant is large.
- Foundry integration is product-critical because Solar exists to improve Solidity developer feedback loops.
- Codegen work must grow around a proof path: MIR infrastructure, differential bytecode/runtime testing, and per-pass fixtures.
- Performance matters only after the relevant correctness oracle passes for the same compiler stage.
- PRs should be reviewable by upstream maintainers: source commits, omitted work, exact commands, proof boundary, limitations, and next dependency must be explicit.

## Campaign Shape

Do not force Solar into tiny tasks. Plan in this hierarchy:

```text
work package -> branch train -> reviewable PR slice -> verifier gates
```

A coherent compiler PR may touch production code, fixtures, corpus runners, expected outputs, and oracle infrastructure together when they prove one semantic invariant. Issues should reflect that scope: each component issue captures the change, the approach, the testing, and the related issues without inventing scope-class enums or LOC budgets.

Branch trains:

- Oracle train: Standard JSON harness, normalized diff artifacts, bytecode comparator, Foundry corpus, CI/check reporting.
- Typeck train: solc test enablement, argument validation, implicit conversions, diagnostics, pass-rate movement.
- Codegen train: MIR text/validator/pass infra, liveness, phi elimination, stack scheduler, assembler, complex lowering.
- Corpus train: fixture generator, bug-pattern corpus, reduced failing cases, xfail pruning.
- Advisory train: performance, LSP, docs, and speculative research when they do not block correctness.

Issue projection policy:

- Issues are control-plane memory for coordination, not proof of progress.
- Prefer the fewest issues that preserve ownership, dependency order, and reviewability. A campaign control issue is useful when it links the current branch train, baseline, proof boundaries, and open PRs; it is not mandatory before the first worker edits.
- Component issues should be generated from the actual plan and current repo/upstream state, not from this file's phase headings. Good issues name a reviewable compiler invariant, the branch/source material if any, the proof tier, and the next dependency.
- Follow-up issues should come from blocked handoffs, review comments, CI failures, corpus deltas, upstream movement, or completed PR evidence.

The planner should keep issues current as memory. Closed issues should explain what was proven, what remains, and which next issue owns the dependency. If generated issues are vague, phase-shaped, or disconnected from code work, skip issue projection and dispatch a better localized task instead.

## Organizer Completion Brief

This brief distills an independent upstream/repo study into the project context the autonomous harness should ingest. The harness is not expected to ignore PADS.md; PADS.md and AGENTS.md are the organizer-authored setup files that tell the harness what a strong Solar campaign should understand before it plans, opens issues, dispatches workers, or judges completion.

As of the 2026-05-12 refresh, local fork `main` is `9aad57d6956812b8b9b80a8d097d524fb6d5314d`; upstream `main` is `d79be54cb8ffb398b8185d1c3c12b387c745835c`; upstream `feat/codegen-mir` is `69d2521c02d5d4ca63c8ba2598b2d67bdf099280`. Solar `main` is best understood as a fast Rust Solidity frontend with lexer/parser, AST, diagnostics under `crates/interface`, file resolution, HIR lowering, symbol resolution, ABI/hash emission, and opt-in type checking behind `-Ztypeck`. It does not have production-ready middle-end/backend codegen on `main`. Codegen exists as active upstream source material on `feat/codegen-mir` and PR `#693`, with PR `#760` exposing real Solar-vs-solc runtime mismatches.

Cheap current corpus facts from the May 12 audit: `tests/ui` has about 194 `.sol` fixtures; typeck UI has about 75 `.sol` fixtures; the solc syntax corpus has about 3,499 `.sol` files; `TypeError` annotations appear in about 1,542 syntax-test files; libyul has about 1,204 `.yul`/`.sol` test files. Treat these as refreshable audit facts, not permanent truth.

The completion path is:

1. Freeze objective oracles: pinned `solc 0.8.31`, UI tests, solc syntax/type-error corpus, project corpus, Foundry runtime equivalence, and benchmark harnesses.
2. Finish frontend/typechecker measurement and targeted semantic fixes.
3. Make Standard JSON and project replay the integration front door.
4. Resolve the Yul/inline-assembly boundary enough for real projects.
5. Extract MIR/codegen from upstream as a dependency-ordered branch train, never as a wholesale merge.
6. Build bytecode/runtime equivalence infrastructure early, even if it starts red/xfail.
7. Optimize only after the corresponding correctness surface is measurable.
8. Use speculative compiler research only behind flags, isolated paths, fixtures, or advisory oracles.

Reference branch-train candidates to consider after refresh (examples, not starter tasks):

Refresh upstream refs, fork diffs, open PRs, conflicts, and current CI before selecting from this list. These candidates are context for senior planning, not a backlog to copy verbatim.

- Replicate or cherry-pick `#761` bare integer alias canonicalization with UI test and full CI.
- Strengthen `#737` into a real `-Ztypeck` solc-corpus lane that counts TypeError tests, records xfails, and never calls ignored exits passing.
- Build a typeck parity report for `#615`: pass/fail/ICE/unsupported by category against pinned `solc`.
- Mine active `feat/typeck-*` branches for narrow semantic fixes with solc-linked fixtures.
- Add a real solc-divergence ledger for `#547` with rationale, fixture, and revisit condition.
- Decide the Yul/HIR semantics that make `#415` actionable before mining `#652`.
- Split `#754` into benchmark-harness correctness first, then parser/source-map performance patches with correctness gates.
- Extract minimal Standard JSON input/project replay before any runtime claim.
- Extract MIR core from `#693`: data model, text parser/printer, validator, pass manager, and `solar-mir-opt`.
- Extract liveness and phi elimination with MIR golden tests.
- Extract `#760`-style runtime equivalence as red/xfail infrastructure that reports exact mismatches.

First campaign slices a strong planner should consider after refreshing repo/upstream/CI state:

1. Add a repo-native oracle inventory report for tool versions, corpus counts, hardcoded skip counts, and local CI gate classification. This is measurement-only and should emit deterministic JSON.
2. Split the solc tester into explicit syntax, Yul, and TypeError/typeck lanes with skip reason counters and no "ignored exit == pass" ambiguity.
3. Import a narrow upstream semantic fix such as `#761` only with source attribution, local fixture, and focused typeck/UI evidence.
4. Implement the minimal Standard JSON front door: stdin parse, `language`, `sources`, `settings.stopAfter`, and honest unsupported-field diagnostics.
5. Add Standard JSON diagnostics parity for `errors[]` shape, source locations, severity, code, and formatted message on small invalid-source fixtures.
6. Emit frontend Standard JSON outputs for ABI and method identifiers through existing ABI/hash logic before bytecode claims exist.
7. Prove source identity and import replay for base/include/remapping/allow-path behavior with multi-file fixtures.
8. Add a tiny Foundry build-info replay oracle that captures `forge config --json`, remappings, build-info Standard JSON input, and an explicit unsupported-output ledger without claiming runtime support.
9. Add a TypeError corpus measurement PR with xfail manifest schema, reason taxonomy, before/after counters, and the first reduced mismatch fixture.
10. Emit simple NatSpec `userdoc`/`devdoc` through Standard JSON.
11. Make Yul corpus skip/xfail accounting explicit and report unsupported lowering truthfully.
12. Extract MIR root infrastructure only: data model, text parser/printer, validator, and pass manager.
13. Extract runtime-equivalence skeleton as red/xfail infrastructure with bytecode normalization and mismatch artifact format.

Generated plans should be judged against this organizer brief. A plan that produces many issues but misses Standard JSON, Yul, typeck corpus measurement, runtime equivalence, or upstream branch mining is low quality even if every issue is well formatted.

## Kickoff Quality Contract

A fresh autonomous kickoff is good only if it can plausibly carry the whole completion contract forward without manual steering. The first master issue and task graph should include a dependency-ordered branch train, not a flat backlog. It should cover at least these lanes before the first implementation wave is trusted:

- Reality freeze: tool versions, fork/upstream refs, CI baseline, corpus counts, skip/xfail counts, and exact unavailable tools.
- Frontend and Standard JSON: a concrete Standard JSON input/output invariant, diagnostics shape, source identity, and at least one selected artifact output.
- Typeck and diagnostics corpus: one measured solc syntax/TypeError/Yul slice with counters and a first semantic fix target.
- Foundry/Hardhat input replay: a minimal build-info or Standard JSON replay artifact that proves project-input handling separately from runtime correctness.
- Codegen/runtime branch train: an infrastructure-only MIR/root or runtime-equivalence slice with proof boundaries, even if it starts red/xfail.
- Review and publication: every worker task should know the issue it reports to, the PR boundary it is trying to produce, and the strongest oracle tier it can honestly claim.

If the generated plan only creates frontend measurement chores, only mirrors this file's phase headings, or omits Foundry/Hardhat, runtime-equivalence, and upstream branch mining, preserve the useful context but replan before spending many worker-hours.

## Phase Model

The phases below describe the engineering shape of the project, not the issue list. They are sequenced by what unblocks what. The component issues a planner generates can map across phases when one PR proves an invariant that spans phases. The planner should keep this order unless a blocker requires preparatory work.

### Phase 0: Reality Freeze And Measurement

Make every future claim measurable. Refresh fork/upstream state, CI baseline, corpus counts, toolchain versions, current unsupported surfaces, and branch watchlist. If the harness cannot measure correctness, implementation PRs become motion.

Start here:

- Run `.pads/setup.sh` and capture versions for `rustc`, `cargo`, `cargo nextest`, and any available oracle/advisory tools such as `solc`, `forge`, `anvil`, `jq`, `uv`, Node/npm/pnpm, and CodSpeed. Missing oracle tools should mark the affected lanes unavailable, not fail kickoff.
- Refresh fork and upstream state: current fork `main`, upstream `main`, open fork PRs, watched upstream PRs, watched branches, and latest CI status.
- Classify current CI failures into required, advisory, environmental, and known-baseline-red. Do not start implementation work until baseline failures are explained.
- Build the first compatibility matrix skeleton from this file: tracks, corpora, oracles, known unsupported features, proof tiers, and current unknowns.
- Record current corpus counts for Solar native UI tests, `TESTER_MODE=solc-solidity`, `TESTER_MODE=solc-yul`, and any available `-Ztypeck` exposure.
- Decide whether each upstream watchlist item is `track`, `extract`, `port`, `cherry_pick`, or `ignore_until_cited`.

Work packages:

- Current-state refresh artifact: fork/upstream/CI/corpus/watchlist snapshot.
- Baseline oracle audit: which commands work locally, which are missing tools, which are too expensive for every PR, and which are advisory.
- Compatibility matrix scaffold: rows for parser, AST, diagnostics, typeck, Standard JSON, ABI, NatSpec, source maps, metadata, bytecode, runtime, Foundry/Hardhat, performance, fuzz.
- Machine-readable xfail/skip ledger schema: reason, issue/track, first-seen commit, corpus source, oracle id, owner track, and revisit condition.
- Standard JSON subset contract: the first supported fields, normalized diff schema, unsupported-field diagnostic shape, and pinned `solc` binary provenance.
- Upstream import provenance schema: source PR/branch/commit, imported commits, omitted commits, local conflicts, attribution, proof tier, and reason the slice is independently safe.
- Unsafe-work queue: dependency update PRs, MSRV hazards, broad codegen branches, workflow edits, snapshot-only changes, stale LSP branches.

Oracles:

- `.pads/setup.sh`
- `cargo check --workspace`
- `cargo nextest run --workspace`
- `cargo uitest`
- `TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests`
- `TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests`

Done for this phase:

- The planner can name the current baseline without guessing.
- Every later implementation task has a real command, corpus, source issue, or blocker to point at.
- No work starts from stale branch/CI/corpus assumptions.

### Phase 1: Baseline Frontend Correctness

Make Solar credible on parse, diagnostics, AST, ABI, NatSpec, Standard JSON frontend output, and semantic/type checking. Prioritize solc comparison, UI fixtures, TypeError corpus exposure, and exact proof boundaries.

Start here:

- Use `#615`, `#663`, and `#737` to expose a narrow TypeError corpus slice under `-Ztypeck`.
- Pick one parser/diagnostic/typeck mismatch where `solc 0.8.31` gives a clear reference result.
- Prefer fixture-backed deltas: a failing input, a focused implementation change, and a command that proves the behavior.
- Build Standard JSON frontend parity from inputs that only require parser/sema outputs before bytecode exists.
- Keep exact diagnostic prose as advisory until error code, severity, and source span parity are stable.

Work packages:

- Typeck corpus lane: category selection, xfail manifest, before/after counts, reduced UI fixtures, first semantic fixes.
- Diagnostics lane: error code, severity, source span, help/note style, UI fixture updates paired with source changes.
- AST/ABI/NatSpec lane: Standard JSON selected fields, ABI shape/order/selectors, devdoc/userdoc/custom tags, method identifiers.
- Import/path lane: source unit names, remappings, base/include/allow paths, metadata-sensitive path behavior.
- Parser/Yul lane: accept/reject parity for selected Solidity and Yul syntax corpus directories.

Oracles:

- `cargo uitest`
- focused `cargo nextest` package tests
- `TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests`
- Standard JSON normalized field diff against pinned `solc`

Done for this phase:

- Frontend compatibility is tracked by matrix row and corpus count, not anecdotes.
- Typeck changes include solc reference behavior and owned fixtures.
- No frontend PR overclaims runtime or bytecode correctness.

### Phase 2: Codegen And Runtime Correctness

Move from frontend correctness to executable EVM output through MIR infrastructure, HIR-to-MIR lowering, liveness, phi elimination, stack scheduling, assembler, bytecode diff, and runtime equivalence. Do not optimize before runtime oracles exist.

Start here:

- Treat `feat/codegen-mir`, `#693`, and `#749` as source material. Extract dependency-ordered slices; never merge wholesale.
- Start with infrastructure that can be tested before full bytecode exists: MIR text format, parser/printer, validator, pass manager, and `solar-mir-opt`.
- Build the equivalence harness early even if only a tiny runtime corpus is supported.
- Land correct-but-naive implementations before optimized versions. For example, naive spilling before heuristic stack scheduling.

Work packages:

- MIR root lane: data model, text format, roundtrip tests, validator, pass manager.
- HIR-to-MIR lane: arithmetic, locals, returns, branches, loops, basic builtins, then complex types.
- Dataflow lane: liveness, phi elimination, stack height model, spill correctness.
- Assembler lane: labels, jumps, link refs, metadata control, bytecode object shape.
- Equivalence lane: normalized bytecode diff, metadata stripping, deploy/call fixtures, runtime comparison against solc.
- Foundry runtime lane: mixed compilation harness, not only `FOUNDRY_SOLC=solar`.

Oracles:

- MIR roundtrip and validator tests.
- Bytecode diff on fixed settings with metadata controlled.
- Runtime differential in revm/Anvil/Foundry comparing returns, reverts, logs, state, created contracts, and gas class.

Done for this phase:

- Every codegen PR states whether it is infrastructure-only, bytecode-diff-backed, or runtime-differential-backed.
- The codegen branch train can merge bottom-up with stack-level verification.
- No optimizer work lands before the correctness surface it optimizes is measurable.

### Phase 3: Production Project Compatibility

Compile real Solidity projects and framework build-info inputs. Prove remappings, profiles, import behavior, build-info, artifact shape, and selected Standard JSON fields before claiming Foundry or Hardhat support.

Start here:

- Use small Foundry fixture projects and `forge-std` before large corpora.
- Capture `forge config --json`, `forge remappings`, `foundry.toml`, compiler settings, output selection, and build-info inputs.
- Reproduce solc Standard JSON input and output first. Solar support can begin with parser/sema/output-shape subsets and explicit unsupported fields.
- Do not count `solar $(forge remappings) file.sol` as Foundry support; it proves only direct frontend ingestion.

Work packages:

- Foundry fixture matrix: profiles, remappings, libs, include paths, allow paths, pinned solc, optimizer, EVM version, `via_ir`, metadata settings, extra outputs.
- Standard JSON replay: minimal input support, source loading, diagnostics JSON, selected output fields, unsupported-output diagnostics.
- Artifact compatibility: ABI, storage layout, metadata, method identifiers, source maps, build-info shape.
- Framework corpus: OpenZeppelin, Solady, Solmate, Uniswap, Seaport, Aave, Compound, Chainlink, ENS.
- Hardhat parity: artifacts, build-info, `sourceName`, `inputSourceName`, profiles, compiler settings.

Oracles:

- `forge config --json` in a selected fixture or project with a real `foundry.toml`
- `forge remappings` in a selected fixture or project with real remappings
- `forge build --force --build-info --build-info-path out/build-info` in a selected fixture or project
- `solc --standard-json < input.json`
- future `solar --standard-json < input.json` after the Solar front door exists
- normalized diff for selected fields

Done for this phase:

- Solar can ingest real project compiler inputs on declared surfaces.
- Unsupported fields are explicit and stable.
- Every corpus PR distinguishes project ingestion from compiler correctness.

### Phase 4: Performance

Preserve and improve Solar speed after the relevant correctness floor exists. Profile first, optimize one hot path per PR, and prove performance with same-environment baseline and real corpora.

Start here:

- Freeze baseline and head commands in the same environment.
- Capture CPU model, target triple, Rust version, Solar commit, solc version where relevant, corpus commit, and benchmark command.
- Profile before optimizing. A performance task without a hot path is a research task, not an implementation task.
- Pair every speedup with the correctness oracle for the optimized stage.

Work packages:

- Benchmark harness correctness: ensure CodSpeed/Criterion/iai commands measure the intended stage.
- Frontend hot paths: lexer cursor, tokenization, parser advancement, string/unescape handling, source map creation.
- Sema hot paths: symbol interning, AST/HIR allocation, resolver maps, type checker caches, clone reduction.
- Project latency: import graph scheduling, file IO, remapping cache, Standard JSON replay, warm/no-op/small-edit builds.
- Memory: peak RSS, allocation count, arena locality, string interning, thin slices.

Oracles:

- correctness stage oracle for changed subsystem
- `cargo codspeed build && cargo codspeed run`
- `cargo bench -p solar-bench --bench iai`
- Criterion benchmark command for the target stage
- optional `/usr/bin/time -v` or profiler artifact

Done for this phase:

- Performance claims include baseline, head, metric direction, noise policy, profiler evidence, and correctness proof.
- Benchmark harness changes are separate from implementation speedups unless the PR clearly scopes both.

### Phase 5: Speculative Research

Explore new compiler architecture only behind safe boundaries: flags, isolated paths, experiments, minimized fixtures, and advisory oracles. Promote to production only after a local invariant and proof path exist.

Start here:

- Mine failed differentials, fuzz crashes, LSP pain points, gas regressions, and corpus failures for research questions.
- Keep research under `research/`, feature flags, draft exploratory PRs, or isolated tools until the idea has fixtures and proof boundaries.
- Ask: what invariant would make this safe? What artifact can show it? What oracle would reject a bad version?

Work packages:

- IR architecture: typed HIR/MIR/eMIR with explicit storage/memory/calldata regions and EVM effects.
- Optimization research: pass manager invariants, profitability model, SCCP, CSE, DCE, jump threading, peephole synthesis.
- Formal methods: rewrite validation, SMT-backed lowering rules, counterexample fixtures.
- Fuzz/metamorphic: grammar generation, solc corpus mutation, equivalent transforms, reducers.
- Incremental compilation: query graph, invalidation explanations, clean-vs-incremental equivalence.
- LSP feedback: diagnostics, completions, go-to-def, type explanations, latency budgets.
- EVM-specific analysis: stack depth, memory expansion, warm/cold storage, transient storage, revert data, gas models.

Oracles:

- minimized fixture or counterexample
- metamorphic relation that fails on a bad implementation
- advisory benchmark or gas snapshot
- proof artifact or solver counterexample
- clean-vs-incremental equivalence

Done for this phase:

- Speculative ideas either graduate into implementation tasks with exact files/oracles or are rejected with rationale.
- Production compiler paths remain protected from unproven research prototypes.

## Compatibility Matrix

Track compatibility by compiler version, EVM version, optimizer mode, corpus, and feature family. Do not use a single "compatible" boolean.

- CLI and Standard JSON: `language`, `sources`, `settings`, `outputSelection`, `errors`, `contracts`, `evm`, `metadata`, `storageLayout`, `ir`, `irOptimized`, `generatedSources`.
- Import and path semantics: source-unit names, `base_path`, `include_path`, `allow_paths`, remappings, context remappings, metadata-sensitive paths.
- Parser and AST: grammar, comments, NatSpec attachment, source spans, source IDs, node order, `stopAfter: "parsing"`.
- Diagnostics: `errors[]` type, component, severity, `errorCode`, message, formatted message, primary/secondary locations.
- Typechecker: overloads, inheritance linearization, data locations, mutability, conversions, ABI coder, custom errors, libraries, modifiers, free functions, UDVT, transient storage.
- ABI and NatSpec: selectors, tuple components, events/errors, public getters, userdoc/devdoc, custom tags.
- Yul and inline assembly: strict assembly, Yul grammar, dialect by hardfork, builtins, `verbatim`, source-map behavior.
- Optimizer assumptions: optimizer details, `runs`, always-on passes, `viaIR` behavior, Yul optimizer sequences.
- Bytecode and runtime: creation/runtime bytecode, CBOR metadata, library placeholders, immutable refs, source maps, gas estimates, reverts, panics, logs, state changes.
- Foundry and Hardhat: `foundry.toml`, remappings, profiles, compiler selection, build-info, artifacts, `sourceName`, `inputSourceName`, extra outputs.

## Correctness Oracle Ladder

Passing a lower tier only proves that tier. Do not overclaim.

| Tier | Oracle | What it proves |
| --- | --- | --- |
| T0 | fmt, typos, generated-file guard | style hygiene only |
| T1 | cargo check/build/clippy | Rust compile/lint health only |
| T2 | cargo nextest/doc tests | local behavior covered by owned tests |
| T3 | cargo uitest | owned diagnostics and UI output |
| T4 | solc parser/Yul corpus | accept/reject/parser corpus behavior |
| T5 | Standard JSON frontend parity | selected errors/AST/ABI/NatSpec/metadata/source-map fields |
| T6 | Foundry/Hardhat build-info replay | real project input handling and configured subset parity |
| T7 | normalized bytecode diff | codegen output equivalence for exact inputs/settings |
| T8 | revm/Anvil/Foundry runtime differential | deployed behavior over covered calls |
| T9 | fuzz/metamorphic tests | unknown-gap discovery, not absence of bugs |
| T10 | SMT/proof/translation validation | modeled rule or pass correctness only |

Every PR must state strongest passing tier, exact commands, corpus deltas, skipped/deferred checks, and proof boundary.

A PR or patch candidate is not ready for publication if it covers only a minority of its acceptance criteria without naming a concrete external blocker, verifier limitation, or dependency handoff. Partial progress is useful as worker evidence, but the PR bar is a reviewable invariant with honest proof boundaries.

Current oracle caveats:

- `TESTER_MODE=solc-solidity` is parser-oriented today. It does not prove TypeError or typechecker parity because TypeError exits are ignored until a distinct `-Ztypeck`/TypeError lane exists.
- `solc.standard_json.frontend`, `foundry.standard_json`, `solc.bytecode`, and `runtime.equivalence` describe desired proof contracts. They become gateable commands only after the corresponding Solar front door, replay tool, codegen output, or runtime harness exists.
- No PR may claim typechecker movement unless it names `-Ztypeck`, `solc 0.8.31`, the corpus subset, xfail/skip delta, and focused UI fixtures or reduced solc cases.
- No PR may claim Standard JSON parity without naming the supported input fields, unsupported-field diagnostic shape, output fields compared, and comparator normalization.

## Foundry And Hardhat Integration

Foundry support is more than running `solar $(forge remappings) file.sol`.

Surfaces to prove:

- `foundry.toml`: src, test, script, out, libs, cache, profiles, optimizer, `via_ir`, `evm_version`, bytecode hash, libraries.
- Remappings: `foundry.toml`, `remappings.txt`, auto-detected libs, context remappings, longest-prefix wins.
- Compiler selection: `solc_version`, auto-detect, offline, `--use`, pragma resolution.
- Outputs: Forge artifacts, build-info, extra output, metadata, IR, storage layout, source maps, method identifiers.
- Runtime: `forge test`, revm, Anvil, traces, return/revert/log/storage/gas comparison once codegen exists.

`FOUNDRY_SOLC=solar` is a smoke test. The stronger target is mixed compilation: tests compiled with `solc`, the contract-under-test compiled with Solar, then behavior compared in the same EVM.

## Corpus Program

Pin every corpus by commit and track what it proves.

1. Solar native corpus: existing `testdata`, UI fixtures, benchmark fixtures.
2. Solidity upstream corpus: syntax tests, semantic tests, gas tests, SMT tests, ABIJson, ASTJSON, libyul, cmdline/Standard JSON fixtures.
3. Ecosystem corpus: OpenZeppelin, Solady, Solmate, Uniswap, Seaport, Optimism, Aave, Compound, Chainlink, ENS.
4. Framework corpus: Foundry and Hardhat projects with profiles, remappings, build-info, optimizer settings, `viaIR`, EVM versions.
5. Fuzz/minimized corpus: OSS-Fuzz, solc-fuzz, Soltix, Echidna, Medusa, Halmos, hevm findings reduced to deterministic fixtures.
6. Runtime corpus: deployable contracts with scripted calls and state assertions.
7. Performance corpus: cold build, warm no-op, comment-only edit, one leaf edit, shared import edit, remapping change, full project builds.

Skip and xfail policy:

- Skip only when a fixture is inapplicable to the declared target.
- Xfail when applicable but Solar has a known bug or unimplemented feature.
- Every xfail needs reason code, issue/track link, first-seen commit, corpus source, oracle id, owner track, and revisit condition.
- Xpass is a signal. Remove or narrow the xfail and update baseline.
- No directory-wide xfails except bootstrap migrations.
- CI fails on unexplained skip/xfail growth.

## Performance Protocol

Performance work begins only after the relevant correctness oracle passes for the same compiler stage.

Required evidence for every performance PR:

- baseline command and head command run in the same environment
- base SHA, head SHA, `solc` version, `rustc` version, target triple, CPU model
- correctness oracle result for the optimized stage
- primary metric with direction and noise policy
- profile evidence naming the hot path changed
- before/after benchmark table or artifact link
- statement of benchmark fixtures/corpora not modified

Reject improvements that only move a microbenchmark and regress a macro corpus.

## Speculative Research Boundaries

Speculative research is encouraged after it has a safe boundary.

Research starts as a research prompt or experiment, not production implementation. Experiments live behind flags or isolated paths. Every idea needs a local invariant, artifact, or oracle before production work. Speculative PRs are `draft_exploratory` until they produce fixtures, minimized counterexamples, or a gated experiment.

Research lanes:

- typed HIR/MIR/eMIR with explicit storage/memory/calldata regions
- pass manager and preservation invariants
- formal rewrite validation
- differential fuzzing and metamorphic relations
- incremental compilation and invalidation proofs
- LSP feedback loops
- EVM/gas analysis and optimizer research
- EOF/new opcode experiments

## Upstream Context

Use these as reference evidence, then verify local applicability before editing:

- `paradigmxyz/solar#1`: roadmap.
- `#615`: typeck tracker and test policy.
- `#663/#737`: solc TypeError corpus exposure.
- `#687`: MIR/codegen and new IR roadmap.
- `#694-#704`: liveness, phi elimination, stack scheduling, spilling, assembler, complex lowering, optimizers, equivalence harness.
- `#693`: broad codegen draft branch, extract-only.
- `#749`: MIR pass manager/text/validator work merged into `feat/codegen-mir`.
- `#760`: Solar-vs-solc runtime comparison on the codegen branch; currently red and useful as mismatch-reporting infrastructure.
- `#761`: narrow sema fix for bare `uint`/`int` aliases with green upstream CI; good first cherry-pick/import candidate.
- `#415/#652`: Yul/HIR boundary and design caution.
- `#754/#508/#475`: performance harness and frontend perf.
- `#743/#744/#755/#758`: narrow current upstream fixes.
- `#547/#689`: solc divergence caution.

## Work Selection Rules

Good slices:

- A compatibility matrix row becomes measured and actionable.
- A solc corpus category is exposed with before/after counts.
- A typeck/diagnostic mismatch gets a fixture and implementation.
- A Standard JSON field gets supported or explicitly rejected with correct diagnostics.
- A Foundry project/input becomes replayable.
- A MIR/codegen dependency lands with fixtures but no overclaimed runtime behavior.
- A red runtime-equivalence comparator becomes a truthful xfail/mismatch report with exact unsupported cases.
- A performance PR changes one hot path and reports correctness plus benchmark evidence.

Bad slices:

- docs-only work unless it unlocks an oracle
- comment-only or TODO-removal-only patches
- snapshot reblessing without source change
- broad merge of `feat/codegen-mir`
- performance work without correctness gates
- codegen/runtime claims without T7/T8 evidence
- generic cleanup not tied to a compiler oracle or active blocker
- plans, issues, or PRs that treat PADS headings as a backlog instead of using PADS/MD as organizer context for repo-grounded work
- generated issues that restate phase headings instead of proposing reviewable compiler work

## PR Quality Bar

Every PR must include:

- Scope and changed files.
- Linked upstream issue, PR, branch, or source commit when relevant.
- For upstream-mined work: exact source branch/commit/PR, omitted changes, and why the slice is safe independently.
- What behavior changed.
- Why this slice is the current useful dependency.
- Exact commands and outputs.
- For corpus work: before/after counts.
- For solc parity: version, EVM version, optimizer mode, and compared field.
- For performance: baseline commit, benchmark commands, before/after numbers, and profiler summary.
- Strongest passing oracle tier and what it does not prove.
- Known risks and follow-up tasks.

Prefer conventional commit titles unless upstream/fork history clearly uses a different convention.

## Solar House Style

Follow `AGENTS.md` and upstream style:

- Conventional commit titles unless recent repo history clearly says otherwise.
- Diagnostic messages should not end with full stops.
- Use backticks for code identifiers in diagnostics.
- Prefer `sym::name` or `kw::Keyword` over string comparisons where applicable.
- Visitors should call `walk_*` unless intentionally stopping traversal.
- Preserve arena allocation discipline and avoid unnecessary clones on hot paths.
- UI tests live under `tests/ui/`; auxiliary files go in `auxiliary/`.
- UI annotations use `//~ ERROR:`, `//~ WARN:`, `//~ NOTE:`, and `//~ HELP:` with `^` / `v` markers for related lines.
- Benchmark changes must say whether they affect parser, sema, interface, codegen, or end-to-end project latency.

## Memory And Frontier Discipline

Before non-trivial work, search Pads wiki/memory, local repo evidence, and upstream issues/PRs. After work, record:

- what was learned
- what failed
- which corpus or oracle was decisive
- exact follow-up slices
- whether the work belongs to correctness, codegen, performance, or research

When blocked, do not keep reading. Produce a blocker artifact that names the missing dependency and creates the next actionable task.
