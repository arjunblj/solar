---
pads_version: 2
preset: compiler
spec_status: active
last_revised: "2026-05-23"
revision_trigger: reset

tier0:
  project:
    slug: arjunblj/solar
    mission: >
      Run an unattended multi-week autonomous campaign that takes this fork
      of paradigmxyz/solar to State of The Art across three axes: (1) 100%
      Solidity 0.8.x compatibility coverage versus solc 0.8.31 measured by
      pinned solc syntaxTests + semanticTests + Yul tests + ABIJson tests +
      Standard JSON differential, (2) significant performance leadership over
      solc on real developer workflows (target floor: 5x faster end-to-end
      cold build, 10x faster warm/incremental rebuild, 3x faster typeck) with
      paired correctness oracles, and (3) novel compiler capabilities solc
      lacks (incremental compilation via query-based architecture, gas-aware
      codegen passes, EOF/EVM Object Format support, formal-rewrite-validated
      optimizations, continuously running differential fuzzers). The
      harness should KEEP discovering new work as campaigns wrap; do not
      mistake "the initial 12 campaigns are merged" for "the project is
      complete." See the Self-Replenishing Work Loop and Beyond-Solc Frontier
      sections for the explicit replenishment heuristics and ambition ceiling.
      The frontend (~38K LOC across 9 crates) is solid and the type checker
      is gated behind -Ztypeck; the backend sits unmerged on
      paradigmxyz/solar:feat/codegen-mir (PR #693, +196k LOC) and needs to
      be split + replayed onto fork main as reviewable lanes; several major
      frontend features (view/pure checker, control-flow graph, linter,
      bytecode-equivalence harness) do not exist in tree at all and need to
      be authored from scratch. Reference upstream paradigmxyz/solar for
      shape and inspiration only; write nothing back to it. All writes go
      to arjunblj/solar:main. After correctness for a surface is measured,
      beat solc on it on real developer workflows. Use that correctness and
      performance shell as the safe substrate for speculative EVM compiler
      research.
    upstream:
      full_name: paradigmxyz/solar
      policy: reference_only
    baseline_commit: "post-2026-05-23-sync"
    program:
      name: "Solar SOTA autonomous campaign"
      window_days_initial: 14
      window_days_sustained: 60
      autonomy_horizon: "multi-week unattended; the harness should run for weeks without operator steering and KEEP discovering new work as campaigns close"
      target_meaty_prs_per_day: 8
      target_parallel_workers: 6
      campaign_count_initial: 12
      campaign_count_sustained: 25
      write_target: "arjunblj/solar:main only"
      sota_targets:
        coverage: "100% pass on pinned solc syntaxTests + semanticTests + Yul tests + ABIJson tests; zero unexplained xfails"
        performance_floor:
          cold_build_speedup_vs_solc: "5x"
          warm_rebuild_speedup_vs_solc: "10x"
          typeck_speedup_vs_solc: "3x"
          parser_throughput_min_mib_per_s_per_core: 50
        beyond_solc:
          - "incremental compilation (Salsa-style query-based architecture with invalidation explanations)"
          - "gas-aware codegen passes (peephole optimizations not present in solc, profile-guided variable layout)"
          - "EOF / EVM Object Format support (post-Cancun opcode coverage, EOF section validation)"
          - "formal-rewrite-validated optimizations (SMT-backed lowering rules, counterexample fixtures)"
          - "continuous differential fuzzing (always-on against solc with auto-minimized regressions)"
          - "richer diagnostics than solc (rustc-style fix-it suggestions, error-code stable URLs, SARIF emit)"
          - "linter (solar-lint) with composable lint passes that consume the CFG and HIR"
  hard_constraints:
    - All writes go to arjunblj/solar:main. Never open a PR against paradigmxyz/solar; never push to it.
    - Treat upstream paradigmxyz/solar as reference-only. Mine its branches (especially feat/codegen-mir) for shape and slice ordering; re-author each slice fresh against fork main rather than wholesale cherry-picking.
    - One invariant per PR. No batched test ports. No multi-feature bundles. Pair every typeck/sema change with a UI fixture under tests/ui/.
    - Every PR body must include behavior diff vs solc 0.8.31, failing oracle (before), passing oracle (after), and named corpus/fixtures touched.
    - Never claim solc compatibility without naming solc version, EVM version, optimizer mode, corpus, and strongest passing oracle tier.
    - Never claim codegen or runtime correctness without bytecode or runtime equivalence evidence, or explicitly label the PR infrastructure-only.
    - Never claim performance without a correctness gate, same-environment baseline, benchmark command, before/after numbers, and profile evidence.
    - Never edit or rebless UI snapshots without paired source changes and a reviewed semantic before/after.
    - Never grow skip or xfail lists without issue link, reason, owner track, and revisit condition.
    - Never commit corpora caches, generated traces, sandbox artifacts, out/, cache/, benchmark images, or vendored Foundry dependencies.
    - Never bump MSRV, change release/publish config, add top-level workspace dependencies, or make broad dependency updates without human approval.
    - Never merge feat/codegen-mir wholesale; split it into reviewable PRs onto fork main.
    - Never plan Solidity language divergence, pre-0.8 compatibility, or stable Rust library API promises unless upstream maintainer policy explicitly changes.
    - Never claim typechecker parity from parser-only corpus runs; typechecker work needs `-Ztypeck`, fixture IDs, and pinned `solc 0.8.31` evidence.
    - Never dispatch LSP, formatter, doc-generator, rename/refactor, or editor-extension product work before the editor-surface unfreeze criteria are met.
    - Never add hardfork-gated opcode or runtime behavior without naming `evmVersion` and pairing the change with pinned-solc differential or runtime evidence.
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
  epoch: "2026-05-23"
  fork_main_commit: "post-2026-05-23-sync (upstream/main + 12 substantive cherry-picks)"
  upstream_main_commit: "2066d4b8 (chore(deps): weekly cargo update #813)"
  upstream_codegen_mir_pr: "paradigmxyz/solar#693 (+196k LOC across 667 files; SPLIT TARGET)"
  upstream_runtime_equivalence_pr: "paradigmxyz/solar#760"
  organizer_completion_brief: "2026-05-23 Solar 14-day MEATY autonomous campaign: split codegen-mir + ship missing frontend features (view/pure, CFG, linter, CompilerOutput surface) + close measurable typeck/sema deltas"
  refresh_before_dispatch: true
  refresh_items:
    - upstream main HEAD (changes daily; expect new dani/typeck-* slices)
    - upstream feat/codegen-mir branch tip (PR #693; refresh before each codegen split slice)
    - upstream open PRs (avoid colliding with #803, #815, #816, #801, #812 and any new dani/* branches)
    - active maintainer branches under dani/* and feat/* (do not touch their files)
    - fork open PRs (the harness should keep <= 6 open at any time; close duplicates aggressively)
    - latest CI status on fork main
    - corpus counts and skip/xfail deltas in tools/tester/
    - whether generated plan/issues/prompts use this completion brief to produce repo-grounded work
  shipped_during_sync_2026_05_23:
    - "rust-toolchain.toml pinned to 1.95.0 (upstream removed; fork keeps to match workspace MSRV)"
    - "tools/tester baseline ledger emit (#2510 cherry-picked)"
    - "scripts/pads/baseline-ledger.py emit (#2479 cherry-picked)"
    - "Mode::SolcSolidityTypeck registered with should_skip_typeck (#2509, #2847ca2a cherry-picked)"
    - "solc-solidity-typeck baseline count buckets for unsupported/xfail (#2525 cherry-picked)"
    - "corpus oracle inventory + skip/xfail ledger reporting (#2212 cherry-picked)"
    - "--standard-json stdin handling in CLI (#2511 cherry-picked)"
    - "Standard JSON diagnostics shape (#2504 cherry-picked)"
    - "typos.toml exempts PADS.md (#f055a88c cherry-picked)"
    - ".gitignore excludes .pads-artifacts/baseline.json (#813f95b7 cherry-picked)"
    - "Port one upstream TypeError fix (#2485 cherry-picked, applied to current ty/mod.rs)"

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
    - id: editor-surface
      source: paradigmxyz/solar
      upstream_ref: issues/394
      mode: track
      priority: low
      related: ["paradigmxyz/solar#394", "paradigmxyz/solar#401", "paradigmxyz/solar#417", "paradigmxyz/solar#418", "foundry-rs/foundry#11619"]
      notes: "Reference-only until upstream basic LSP is merged/released, lifecycle and symbol-table blockers close, and frontend diagnostics are trustworthy enough for editor consumption."
    - id: natspec-lowering
      source: paradigmxyz/solar
      upstream_ref: pull/567/head
      mode: track
      priority: medium
      related: ["paradigmxyz/solar#567"]
      notes: "NatSpec belongs to compiler artifact compatibility (`userdoc`/`devdoc`), not a standalone doc-generator product."
    - id: foundry-solar-adoption
      source: foundry-rs/foundry
      upstream_ref: open-issues-and-prs
      mode: track
      priority: high
      related: ["foundry-rs/foundry#9317", "foundry-rs/foundry#11307", "foundry-rs/foundry#11652", "foundry-rs/foundry#12721", "foundry-rs/foundry#10965"]
      notes: "Foundry public adoption currently centers on AST/context/lint/backtrace/flatten and LSP, not a merged full Solar codegen replacement."

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
    required_oracles: [perf.phase-report, perf.codspeed, perf.iai]
  - id: fuzz-metamorphic
    name: Fuzz and Metamorphic Testing
    priority: medium
    status: active
    scope: ["tools/**", "testdata/**", "crates/**", "fuzz/**"]
    required_oracles: [fuzz.regression-minimized, fuzz.differential]
  - id: speculative-research
    name: Speculative Compiler Research
    priority: research
    status: active
    scope: ["research/**", "crates/**", "tools/**"]
    required_oracles: [research.artifact]
  - id: view-pure-checker
    name: View/Pure (state-mutability) checker
    priority: critical
    status: active
    scope: ["crates/sema/src/typeck/**", "tests/ui/typeck/view_pure/**", "tests/ui/typeck/state_mutability/**"]
    archetypes: [sema-feature, oracle-port-from-solc]
    required_oracles: [cargo.uitest, cargo.nextest]
    upstream_tracking: ["paradigmxyz/solar#1"]
    notes: "Entirely missing in tree. Reference solc libsolidity/analysis/ViewPureChecker.{h,cpp}. Author crates/sema/src/typeck/view_pure.rs from scratch covering state read/write inference, asm opcode mutability, external call propagation, modifier propagation, override stricter rule, and rustc-style suggestions. Pair every slice with viewPureChecker/ corpus fixtures."
  - id: control-flow-graph
    name: CFG and flow-sensitive checks
    priority: high
    status: active
    scope: ["crates/sema/src/cfg/**", "crates/sema/src/lib.rs", "tests/ui/resolve/**"]
    archetypes: [sema-feature]
    required_oracles: [cargo.nextest, cargo.uitest]
    upstream_tracking: ["paradigmxyz/solar#1"]
    notes: "Roadmap calls out CFG as unchecked. Build crates/sema/src/cfg/ from HIR with dominators, then layer unreachable-statement, returns-on-all-paths, unused locals, unused params, immutable assignment-on-all-paths, try/catch paths, yul fall-through. Foundation for view/pure on conditional branches and most lints."
  - id: solar-lint
    name: solar-lint crate foundation
    priority: medium
    status: active
    scope: ["crates/lint/**", "crates/config/src/**", "crates/sema/src/lib.rs", "crates/solar/src/lib.rs"]
    archetypes: [new-crate]
    required_oracles: [cargo.nextest]
    upstream_tracking: ["paradigmxyz/solar#1"]
    notes: "Crate does not exist on main. Stand up crates/lint/ with driver hook, lint pass framework with HIR/CFG access, lint registry plus #[allow]/#[deny] natspec attributes, config TOML in solar-config. Initial passes: unused state vars, builtin shadowing, empty contract body, i++ vs ++i, missing docstring, unchecked external-call return value."
  - id: codegen-mir-split
    name: Split feat/codegen-mir into reviewable PRs on fork main
    priority: critical
    status: active
    scope: ["crates/codegen/**", "crates/sema/src/mir/**"]
    archetypes: [codegen-replay]
    required_oracles: [cargo.build, cargo.nextest]
    upstream_tracking: ["paradigmxyz/solar#693", "paradigmxyz/solar#749", "paradigmxyz/solar#760", "feat/codegen-mir"]
    notes: "The big 14-day prize. Replay the +196k LOC of MIR + EVM codegen on paradigmxyz/solar:feat/codegen-mir into >=20 child PRs on arjunblj/solar:main. Each child re-authors one coherent slice (NOT wholesale cherry-pick); use feat/codegen-mir as semantic reference. Sequence: pass manager - MIR types - MIR builder - lowering passes - EVM emit - stack scheduling - assembler. Each child must `cargo build` and pass relevant tests independently. Read upstream branch tip before each slice; expect the branch to keep moving."

priority_order:
  - codegen-mir-split
  - view-pure-checker
  - typeck-corpus
  - abi-natspec-sourcemaps
  - standard-json
  - parser-ast-diagnostics
  - control-flow-graph
  - yul-hir-boundary
  - mir-codegen
  - runtime-equivalence
  - solar-lint
  - foundry-hardhat
  - compatibility-matrix
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
  - The upstream vision is the starting map, especially the codegen/MIR roadmap on `paradigmxyz/solar` (PR #693, issue #687, runtime-equivalence work in PR #760) and the active typeck/sema cadence. The autonomous campaign should first reach baseline correctness and observability, then surpass upstream performance and research quality on this fork with explicit oracles.
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
  - id: snapshots.clean
    kind: shell
    tier: prerequisite
    time_budget_s: 30
    command: "git diff --exit-code -- ':(glob)tests/ui/**/*.stderr' ':(glob)tests/ui/**/*.stdout' ':(glob)**/*.snap'"
    notes: "Detects uncommitted snapshot drift after generators/tests. Committed snapshot diffs still require paired source changes and semantic review."
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
    status: desired_until_durable_fuzz_harness_exists
    corpus_ref: fuzz-minimized
    compare: [accept_reject, runtime_equivalence, diagnostic_category]
  - id: fuzz.regression-minimized
    kind: artifact
    tier: advisory
    status: desired_until_reducer_and_regression_fixture_flow_exists
    corpus_ref: fuzz-minimized
    notes: "Current fuzz scripts discover/generate inputs; promotion to a gate requires reducer invocation, deduplication, minimized fixture placement, and expected outcome classification."
  - id: perf.phase-report
    kind: artifact
    tier: advisory
    status: required_for_performance_claims_until_repo_native_tool_exists
    corpus_ref: performance-corpus
    notes: "Every performance PR must name environment, corpus manifest, baseline/head commands, metrics, profile evidence, and the correctness oracle gating the optimized stage."
  - id: coverage.report
    kind: artifact
    tier: advisory
    status: not_a_gate_until_coverage_tooling_exists
    notes: "No public CI coverage gate exists; coverage data is useful context only until a repo-owned command is added."
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
  - "Solar passes 100% of pinned solc 0.8.31 syntaxTests with -Ztypeck and TESTER_MODE=solc-solidity-typeck. Every xfail has a linked issue, reason, owner track, and revisit condition."
  - "Solar passes 100% of pinned solc Yul tests with TESTER_MODE=solc-yul."
  - "Standard JSON I/O (input + output) matches solc 0.8.31 on the canonical corpus for sources, settings, errors[], contracts[].abi, contracts[].evm.bytecode, contracts[].evm.deployedBytecode, contracts[].metadata, contracts[].userdoc, contracts[].devdoc, contracts[].storageLayout, contracts[].evm.methodIdentifiers, sourceMaps."
  - "ABI emission covers referenced events, referenced custom errors, internalType for tuples/structs/UDVTs, anonymous events, indexed-parameter ordering, error custom-data layout, and emit-side selector clash detection."
  - "View/pure (state-mutability) checker matches solc on the viewPureChecker/ corpus end-to-end with rustc-style suggestions."
  - "Control-flow graph + flow-sensitive checks ship: unreachable, returns-on-all-paths, unused locals, unused params, immutable assignment-on-all-paths, try/catch paths, yul fall-through."
  - "feat/codegen-mir is replaced by >=20 merged child PRs on fork main covering pass manager - MIR types - MIR builder - lowering passes - EVM emit - stack scheduling - assembler. `cargo build -p solar-codegen` succeeds on fork main."
  - "Bytecode-equivalence harness compares Solar vs solc 0.8.31 on a pinned >=20-contract runtime corpus; mismatches are recorded as regression fixtures."
  - "Constant evaluator handles every match arm in eval.rs (no commented stubs, no UnsupportedBinaryOp reachable on legitimate inputs)."
  - "Diagnostic infrastructure has zero unimplemented!() in human.rs / context.rs. SARIF + URL mapping + Suggestion/Applicability rendering ship. Stable error-code registry is the source of truth for every diagnostic emitted."
  - "solar-lint crate exists and ships >=4 lints (unused state vars, builtin shadowing, empty contract body, unchecked external-call return value)."
  - "Performance leadership: per-stage Solar-vs-solc-0.8.31 SOTA report shows >=5x cold build, >=10x warm rebuild, >=3x typeck on the pinned performance corpus. Each speedup PR ships paired correctness oracle."
  - "Incremental compilation (--unstable-incremental) wires parser + resolver + typeck queries with clean-vs-incremental equivalence tests; warm rebuild on a one-leaf-edit smoke is >=10x faster than cold."
  - "Continuous differential fuzzer runs hourly/daily; each surfaced regression lands as a minimized fixture under tests/regressions/ with paired Solar-side fix or upstream issue link."
  - "Speculative research lanes (gas-aware codegen, EOF support, formal-rewrite-validated optimizations, language extensions) produce fixtures, minimized counterexamples, or gated experiments without polluting production paths. Each speculative result either graduates to a production lane with implementation tasks + oracles, or is rejected with rationale."

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
    research_synthesis:
      evidence_level: "Organizer-supplied deep research synthesis, to be refreshed against live upstream before branch dispatch."
      intent:
        - "Solar is a fast modular Rust Solidity compiler stack for CLI and library use, but upstream still marks it not feature-complete and not production-ready."
        - "The compatibility target is Solidity 0.8.x, not pre-0.8 Solidity and not language divergence."
        - "The frontend definition of done is measured parity against pinned solc, with typechecker work anchored to `solc 0.8.31` and `-Ztypeck`."
        - "Foundry integration matters, but first-wave autonomous work should prove the Standard JSON/process/artifact contract before any full compiler replacement claim."
      reference_only_surfaces:
        - "LSP/editor/formatter/doc-generator work remains monitoring-only until upstream #401/#417/#418 and frontend diagnostics are materially stronger."
        - "Backend/codegen work remains a dependency-ordered reference branch train; #693 is source material, not a merge target."
        - "PGO/BOLT/allocator/interner/cache experiments remain bounded performance research until correctness-gated corpora and phase reports exist."
      high_risk_feature_families:
        - ABI encoding/decoding, selectors, custom errors, and revert data
        - storage layout, transient storage, UDVT storage, custom base slots, and clearing semantics
        - Yul/inline assembly, memory-safe annotations, object-mode Yul, and `verbatim_`
        - constructors, immutables, CREATE2-sensitive bytecode, metadata, library linking, and source maps
        - inheritance linearization, overrides, `super`, modifiers, receive/fallback dispatch, and `try/catch`
        - hardfork-gated opcodes and behavior across Berlin, Shanghai, Cancun, Prague, Osaka, and later forks
    current_state:
      shipped:
        - "lexer/parser, AST, diagnostics, file resolver, HIR lowering, sema/typeck WIP, ABI/hash emission, UI tests, solc syntax and Yul corpus runners"
        - "Symbol resolution incl. using-for (#773), contract members (#793 series), yul lowering (#769), function-pointer typeck (#775, #776 + ongoing dani/typeck-using-function-values)"
        - "Type checker behind -Ztypeck (#774, #780, #800 plus the dani/typeck-* slices on a near-daily cadence)"
        - "Override checker (#685) + NatSpec lowering (#768)"
        - "Standard JSON stdin handling on the fork (#2511 cherry-picked)"
        - "Standard JSON diagnostics shape on the fork (#2504 cherry-picked)"
        - "TESTER_MODE=solc-solidity-typeck mode registration on the fork (#2509, #2525, #2847ca2a cherry-picked)"
        - "Baseline corpus ledger + skip/xfail accounting on the fork (#2479, #2510, #2212 cherry-picked)"
        - "rust-toolchain.toml pin at 1.95.0 (matches workspace MSRV; upstream removed the file but fork keeps it)"
      not_shipped:
        - "MIR / EVM codegen / lowering on main (sits on paradigmxyz/solar:feat/codegen-mir, PR #693, +196k LOC unmerged)"
        - "View/pure (state-mutability) checker (entirely missing - no crates/sema/src/typeck/view_pure.rs)"
        - "Control-flow graph + flow-sensitive checks (unreachable, returns-on-all-paths, unused locals)"
        - "CompilerOutput::StorageLayout / TransientStorageLayout"
        - "CompilerOutput::DevDoc / UserDoc / Metadata / Asm / AsmJson / Ir / FunctionDebug (only Abi + Hashes wired today; emit.rs:53 has todo!() for everything else)"
        - "Standard JSON output emitter (only stdin handling is shipped; outputSelection glob, remappings, libraries, optimizer settings, errors[] secondarySourceLocations all TODO)"
        - "Constant evaluator full coverage (eval.rs has 14 commented ExprKind arms, 9 unsupported binops including Sar/comparisons/logical short-circuit/Ternary)"
        - "Diagnostic suggestion rendering (annotate-snippets v0.11), stable error-code registry, URL mapping (human.rs:165/219, context.rs:193 unimplemented)"
        - "interfaceType + transient storage in ty/mod.rs (unimplemented!())"
        - "solar-lint crate (does not exist on main)"
        - "Bytecode-equivalence test harness (issue #704)"
        - "Foundry build-info replay through Solar"
        - "WASI compat (issue #211)"
      current_default_principle: "Prefer measured gaps and proof-producing tooling over compatibility claims. Encourage MEATY single-invariant PRs (200-1000 LOC, multi-file when justified) over fixture-port chores."
    known_gaps:
      - id: codegen_mir_split
        track: codegen-mir-split
        current_evidence: "paradigmxyz/solar:feat/codegen-mir (PR #693) carries +196,003 LOC across 667 files unmerged. crates/codegen does not exist on fork main."
        next_measurement: "Number of child PRs merged onto arjunblj/solar:main covering pass manager - MIR types - MIR builder - lowering passes - EVM emit - stack scheduling - assembler. Target: >=20 child PRs with `cargo build -p solar-codegen` succeeding."
        proof_boundary: "Each child PR is infrastructure or a single coherent slice with fixtures; do not claim runtime equivalence until codegen output exists for the selected subset and the comparator runs against pinned solc."
      - id: view_pure_missing
        track: view-pure-checker
        current_evidence: "No crates/sema/src/typeck/view_pure.rs file exists. roadmap (#1) explicitly lists view/pure as unchecked."
        next_measurement: "tests/ui/typeck/view_pure/* fixture pass-rate vs solc viewPureChecker/ corpus."
        proof_boundary: "First PR is the pass skeleton (HIR walker producing per-fn EffectiveMutability, no diagnostics yet). Diagnostics + suggestions ship in subsequent slices."
      - id: cfg_missing
        track: control-flow-graph
        current_evidence: "No crates/sema/src/cfg/ exists. Roadmap (#1) lists CFG generation unchecked. Five flow-sensitive checks (unreachable, returns-on-all-paths, unused locals, unused params, immutable assignment-on-all-paths) all depend on it."
        next_measurement: "solar_sema::cfg module compiles with dominators + reverse-postorder. Each downstream flow-sensitive check ships as its own PR."
        proof_boundary: "F1 (skeleton) cannot ship diagnostics; F2 (RPO + dominators) is the foundation; F3 (reachability) unblocks F4 (unreachable diag) and the Campaign A view/pure modifier propagation lane."
      - id: linter_missing
        track: solar-lint
        current_evidence: "No crates/lint/ on main. Roadmap (#1) lists static analysis + lint infrastructure unchecked."
        next_measurement: ">=4 lints shipped in solar-lint. Lint config TOML lives in solar-config. Driver hook fires after typeck + CFG."
        proof_boundary: "Lint registry framework first; #[allow]/#[deny] natspec attributes second; then individual passes (unused state vars, builtin shadowing, etc.)."
      - id: compiler_output_stubs
        track: abi-natspec-sourcemaps
        current_evidence: "crates/sema/src/emit.rs has `emit => todo!()` for unsupported variants. CompilerOutput today emits only Abi + Hashes."
        next_measurement: "Number of CompilerOutput variants emitted (currently 2; target >=8 of StorageLayout, TransientStorageLayout, DevDoc, UserDoc, Metadata, Asm, AsmJson, Ir, FunctionDebug)."
        proof_boundary: "Each new variant ships with at least one tests/ui/abi fixture exercising the JSON shape against pinned solc output."
      - id: const_eval_stubs
        track: typeck-corpus
        current_evidence: "rg 'UnsupportedBinaryOp|// hir::ExprKind::' crates/sema/src/eval.rs returns 25+ matches. Comparisons (Lt/Le/Gt/Ge/Eq/Ne), short-circuit logical (And/Or), Sar, Ternary, Member, Tuple, TypeCall, LitKind::Str, LitKind::Rational all unhandled."
        next_measurement: "Count of remaining UnsupportedBinaryOp arms + commented `// hir::ExprKind::*` arms in eval.rs (target: 0)."
        proof_boundary: "Each PR closes one match arm with paired UI fixture under tests/ui/typeck/const_eval_*."
      - id: typeck_cpp_todos
        track: typeck-corpus
        current_evidence: "rg 'TODO.*solidity/blob/.*libsolidity/analysis' crates/sema/ shows 7 explicit TODO refs to solc TypeChecker.cpp line numbers. ty/mod.rs has interfaceType + transient gaps. ty/print.rs has the richIdentifier parity TODO."
        next_measurement: "Resolved TODO count in checker.rs / ty/mod.rs / ty/print.rs (target: 0 unresolved Solar-side TODOs that reference solc lines)."
        proof_boundary: "Each TODO closure ships as a single PR with the matching solc syntaxTest fixture under tests/ui/typeck/."
      - id: diagnostic_infra_gaps
        track: parser-ast-diagnostics
        current_evidence: "crates/interface/src/diagnostics has unimplemented!() in context.rs:193 and human.rs:165, an old FIXME hack in human.rs:456, partial URL/suggestion paths in human.rs:219."
        next_measurement: "Zero unimplemented!() reachable from human.rs/context.rs. Stable error-code registry with URL mapping. Suggestion/Applicability rustc-style fix-it rendering. JSON emitter parity for secondarySourceLocations. SARIF emitter."
        proof_boundary: "Each diagnostic infra slice ships behind tests/ui/cli fixtures or solar-interface unit tests."
      - id: yul_typeck_tail
        track: yul-hir-boundary
        current_evidence: "#769, #774, #785, #799, #802, #807 shipped the spine. checker.rs:1904 has TODO for external fn-pointer return type in visit_ty. Edge cases keep surfacing per recent dani/* slices."
        next_measurement: "Switch default-required, reachability after revert/return/stop, verbatim builtin signatures, nested-function scoping, label-style identifier rejection, memory-safe alignment checks, const-folding, object/code/data sections each shipped."
        proof_boundary: "Each edge case ships with tests/ui/yul_lowering fixtures."
      - id: standard_json_output_emitter_missing
        track: standard-json
        current_evidence: "Fork ships --standard-json stdin (#2511) but the OUTPUT emitter is not wired. settings.outputSelection glob, remappings, libraries, optimizer settings, evmVersion, errors[] secondarySourceLocations all missing from the response side."
        next_measurement: "tests/ui/cli/standard_json/* coverage. solar --standard-json produces solc-compatible JSON for the canonical test set."
        proof_boundary: "Builds on already-shipped Standard JSON stdin (#2511). Each output-side slice ships with paired solc differential."
      - id: bytecode_equivalence_harness
        track: runtime-equivalence
        current_evidence: "Main lacks production codegen/bytecode output. Issue #704 calls for bytecode-equivalence test infrastructure. tools/equiv-tester does not exist."
        next_measurement: "tools/equiv-tester crate built; tests/codegen-equivalence/ corpus curated (>=20 contracts); per-test ABI-diff (today) + bytecode-diff (after codegen-mir-split lands)."
        proof_boundary: "Infrastructure-only initially (ABI-diff). Bytecode-diff and runtime-diff oracles unblock as codegen-mir slices land."
      - id: performance_leadership_unmeasured
        track: performance-leadership
        current_evidence: "Solar has parser/IAI/CodSpeed benches but no SOTA-targeting phase report comparing Solar vs solc 0.8.31 on identical workloads."
        next_measurement: "Per-stage speedup table vs solc 0.8.31 on pinned corpora (parser, sema, full build, warm rebuild). Targets: 5x cold, 10x warm, 3x typeck."
        proof_boundary: "Each performance PR ships paired correctness oracle + same-environment baseline + before/after numbers + profile evidence."
      - id: incremental_compilation_missing
        track: incremental-compilation
        current_evidence: "Solar recompiles eagerly. No query-based architecture. No invalidation explanations. Warm rebuild on a small edit takes the same wall-clock as a cold build."
        next_measurement: "Salsa-style or hand-rolled query graph with invalidation; warm rebuild / one-leaf-edit latency vs cold build."
        proof_boundary: "Phase 1 is research + design behind a flag; Phase 2 is the query layer for parser+sema; Phase 3 extends to typeck and downstream. Each phase ships with clean-vs-incremental equivalence tests."
      - id: gas_aware_codegen_missing
        track: gas-aware-codegen
        current_evidence: "Codegen is on feat/codegen-mir; gas-aware passes are not in upstream's roadmap. Solar can be cleanly more aggressive than solc on stack scheduling, dead-store elimination, jump threading, and storage-slot packing."
        next_measurement: "Per-contract gas delta vs solc on the runtime-equivalence corpus."
        proof_boundary: "Each gas-saving pass ships behind a flag with paired runtime-equivalence proof and a fixture proving the gas decrease."
      - id: eof_support_missing
        track: eof-support
        current_evidence: "EOF (EVM Object Format, post-Cancun) is not in solc 0.8.31. Solar can ship EOF section validation, EOF call/jumpf/dataload opcodes, and EOF-mode codegen as differentiating capability."
        next_measurement: "EOF section validation passes for the EOF spec test corpus. Codegen emits EOF-formatted bytecode behind --evmVersion=osaka."
        proof_boundary: "Validation first (parser+typeck on EOF-only constructs); codegen later, after the codegen-mir-split campaign delivers MIR+emit infrastructure."
      - id: formal_verification_missing
        track: formal-verification
        current_evidence: "No SMT-backed lowering rule validation in tree. No rewrite-correctness fixtures."
        next_measurement: "Number of validated lowering rules + counterexample fixtures."
        proof_boundary: "Research lane. Each validated rule ships with the SMT artifact + counterexample corpus + a property-based test pinning the invariant."
      - id: continuous_fuzzing_missing
        track: fuzz-leadership
        current_evidence: "Solar has fuzz harnesses but no continuously running differential fuzzer against solc. No auto-minimized regression flow."
        next_measurement: "Per-week fuzzer-discovered regressions added to tests/regressions/ as minimized fixtures."
        proof_boundary: "Each fuzzer-discovered finding ships as a regression fixture + paired Solar-side fix or upstream issue link."
      - id: language_extensions_research
        track: speculative-research
        current_evidence: "Solar can speculate on language extensions (compile-time generics, structural type narrowing, `constexpr`-style evaluation, gas-aware diagnostics) behind feature flags without affecting Solidity 0.8.x compatibility."
        next_measurement: "Each extension ships behind --unstable-feature=<name> with a design RFC, fixtures, and an explicit non-default flag."
        proof_boundary: "Research lane only. Promotion to default requires upstream maintainer policy change AND parity coverage on the existing 0.8.x surface."
      - id: editor_surface_deferred
        track: speculative-research
        current_evidence: "Upstream LSP PR #401 is draft and lifecycle/symbol-table blockers remain open; Foundry LSP integration waits on a Solar release."
        next_measurement: "Monitor upstream #401/#417/#418 and only prototype AnalysisHost-style snapshots after frontend diagnostics are trustworthy."
        proof_boundary: "Reference-only; not first-wave autonomous implementation work."
      - id: feature_matrix_unmeasured
        track: compatibility-matrix
        current_evidence: "High-risk Solidity/EVM families where parser or symbol awareness does not imply semantic, artifact, or runtime support remain unmeasured."
        next_measurement: "Each high-risk family becomes a feature-matrix row with pinned solc fixtures + current Solar support classification + next-measurement hook."
        proof_boundary: "Classification and fixture planning only until differential evidence exists."
    calibration_slices_shipped_2026_05_23:
      # All four calibration slices are now SHIPPED on fork main (cherry-picked
      # from older fork commits during the 2026-05-23 sync). Listed here so
      # the planner does not regenerate them as new tasks.
      - "Oracle inventory + baseline ledger (#2479 baseline-ledger.py + #2510 tools/tester emit + #2212 corpus oracle inventory)"
      - "Minimal Standard JSON front door / stdin (#2511)"
      - "TypeError measurement lane: TESTER_MODE=solc-solidity-typeck (#2509 + #2525 baseline count buckets + #2847ca2a should_skip_typeck)"
      - "Standard JSON diagnostics shape (#2504)"

    # Real file paths that exist in this repo today, indexed by the
    # known_gap id above. The planner MUST cite seed paths from this index
    # rather than inventing plausible-looking paths. Any task whose evidence
    # references files outside this index is mis-localized and should be
    # repaired before dispatch.
    #
    # Last verified against fork main 8d26b642 on 2026-05-18.
    track_files:
      codegen_mir_split:
        primary_files:
          - "crates/codegen/Cargo.toml (NEW; replay from feat/codegen-mir)"
          - "crates/codegen/src/lib.rs (NEW)"
          - "crates/sema/src/mir/ (NEW)"
          - "crates/sema/src/lib.rs (wire codegen module after typeck)"
        relevant_dirs:
          - crates/sema/
          - crates/parse/
        oracle_commands:
          - "cargo build -p solar-codegen"
          - "cargo nextest run -p solar-codegen"
          - "git ls-remote https://github.com/paradigmxyz/solar feat/codegen-mir   # refresh branch tip before each slice"
        slice_order:
          - "MIR core data model + text format + roundtrip tests"
          - "MIR pass manager + validator"
          - "HIR-to-MIR lowering: arithmetic, locals, returns"
          - "HIR-to-MIR lowering: branches, loops, basic builtins"
          - "HIR-to-MIR lowering: complex types (mappings, dynamic arrays, structs)"
          - "Liveness analysis + phi elimination"
          - "Stack height model + spill correctness (naive first)"
          - "Assembler: labels, jumps, link refs, metadata control"
          - "EVM emit: bytecode object shape, runtime/creation distinction"
          - "Stack scheduling (graph coloring or heuristic)"
          - "Optimizer passes: SCCP, CSE, DCE (each as own PR)"
          - "Bytecode-equivalence comparator wiring"
        first_pr_target: |
          Stand up crates/codegen/ with the MIR core data model + text format
          + roundtrip tests. Do not import the entire feat/codegen-mir branch;
          read it as reference and re-author the slice freshly. ~600-1200 LOC.
      view_pure_missing:
        primary_files:
          - "crates/sema/src/typeck/view_pure.rs (NEW)"
          - crates/sema/src/typeck/mod.rs
          - crates/sema/src/typeck/override_checker.rs
        relevant_dirs:
          - testdata/solidity/test/libsolidity/syntaxTests/viewPureChecker/
          - tests/ui/typeck/
        oracle_commands:
          - "cargo nextest run -p solar-sema -- ui::typeck::view_pure"
        slice_order:
          - "A1 pass skeleton: HIR walker producing per-fn EffectiveMutability"
          - "A2 state-read inference (storage + immutable reads)"
          - "A3 state-write inference (assignment / delete / .push / mapping write)"
          - "A4 inline-assembly opcode mutability via #769 lowered HIR"
          - "A5 external-call mutability propagation"
          - "A6 modifier propagation + override stricter rule"
          - "A7 diagnostics with rustc-style suggestion"
          - "A8 mass solc viewPureChecker syntaxTests port (paired source + fixture)"
        first_pr_target: |
          A1 pass skeleton. Author crates/sema/src/typeck/view_pure.rs with a
          HIR walker that classifies every function's EffectiveMutability into
          Pure / View / NonPayable / Payable and records into
          gcx.contract_view_pure_results, but does NOT emit diagnostics yet.
          Wire a single placeholder UI test under tests/ui/typeck/view_pure/skeleton.sol.
          ~400-700 LOC.
        references:
          - "solc libsolidity/analysis/ViewPureChecker.{h,cpp}"
      cfg_missing:
        primary_files:
          - "crates/sema/src/cfg/mod.rs (NEW)"
          - "crates/sema/src/cfg/builder.rs (NEW)"
          - "crates/sema/src/cfg/dominators.rs (NEW)"
          - crates/sema/src/lib.rs
        relevant_dirs:
          - tests/ui/resolve/
          - tests/ui/typeck/
        oracle_commands:
          - "cargo nextest run -p solar-sema -- sema::cfg"
          - "cargo nextest run -p solar-sema -- ui::resolve"
        slice_order:
          - "F1 solar_sema::cfg module + Cfg<'gcx> + BasicBlock + Terminator from HIR Block"
          - "F2 reverse-postorder + dominators"
          - "F3 reachability propagation"
          - "F4 unreachable-statement diagnostic"
          - "F5 returns-on-all-paths check"
          - "F6 unused local variable detection"
          - "F7 unused / shadowed parameter warning"
          - "F8 immutable assignment-on-all-constructor-paths"
          - "F9 try/catch path analysis"
          - "F10 yul fall-through validation"
        first_pr_target: |
          F1 skeleton. Add crates/sema/src/cfg/mod.rs with `Cfg<'gcx>`,
          `BasicBlock`, `Terminator` (Goto / SwitchInt / Return / Revert / Stop /
          ExternalCall), and a builder that walks HIR blocks. No diagnostics
          yet; just produce CFGs and dump them via a debug helper. ~600-900 LOC.
        references:
          - "rustc compiler/rustc_mir_build/src/build/cfg.rs (shape reference)"
          - "solc libsolidity/analysis/ControlFlowGraph.{h,cpp}"
      linter_missing:
        primary_files:
          - "crates/lint/Cargo.toml (NEW)"
          - "crates/lint/src/lib.rs (NEW)"
          - "crates/lint/src/registry.rs (NEW)"
          - "crates/lint/src/passes/ (NEW)"
          - crates/sema/src/lib.rs
          - crates/solar/src/lib.rs
        relevant_dirs:
          - tests/ui/lint/
        oracle_commands:
          - "cargo build -p solar-lint"
          - "cargo nextest run -p solar-lint"
        slice_order:
          - "J1 solar-lint crate skeleton + driver hook from compiler stage"
          - "J2 lint pass framework (LateContext-style with HIR + CFG access)"
          - "J3 lint registry + #[allow]/#[deny] natspec attribute parsing"
          - "J4 lint config TOML in solar-config"
          - "J5 lint #1: unused state variables"
          - "J6 lint #2: shadowing builtin / global names"
          - "J7 lint #3: empty contract body / orphan modifier"
          - "J8 lint #4: i++ vs ++i in for-loops"
          - "J9 lint #5: missing docstring on public function/event"
          - "J10 lint #6: unchecked external-call return value"
        first_pr_target: |
          J1: stand up crates/lint/ with Cargo.toml + driver hook from
          crates/sema. The driver runs after typeck. Produce a stub lint that
          emits one info-level diagnostic so the wiring is exercised. ~300-500 LOC.
      compiler_output_stubs:
        primary_files:
          - crates/config/src/lib.rs
          - crates/sema/src/emit.rs
          - crates/sema/src/builtins/members.rs
        relevant_dirs:
          - tests/ui/abi/
        oracle_commands:
          - "cargo nextest run -p solar-sema -- ui::abi"
        slice_order:
          - "C1 CompilerOutput::StorageLayout + JSON emit"
          - "C2 CompilerOutput::TransientStorageLayout"
          - "C3 CompilerOutput::DevDoc (uses #768 NatSpec lowering)"
          - "C4 CompilerOutput::UserDoc"
          - "C5 CompilerOutput::Metadata (sources / settings / compilerVersion)"
          - "C6 CompilerOutput::Asm (legacy text placeholder)"
          - "C7 CompilerOutput::AsmJson (placeholder JSON shape)"
          - "C8 CompilerOutput::Ir (HIR-shaped text dump)"
          - "C9 CompilerOutput::FunctionDebug"
        first_pr_target: |
          C1: wire StorageLayout. Add the variant to CompilerOutput, route
          through emit.rs, build the JSON shape from the existing storage
          slot computations. Pair with tests/ui/abi/storage_layout_basic.sol
          + .stdout pinned against solc --storage-layout. ~400-700 LOC.
      const_eval_stubs:
        primary_files:
          - crates/sema/src/eval.rs
        relevant_dirs:
          - tests/ui/typeck/
        oracle_commands:
          - "cargo nextest run -p solar-sema -- ui::typeck::const_eval"
        slice_order:
          - "D1 comparison ops (Lt/Le/Gt/Ge/Eq/Ne)"
          - "D2 short-circuit && and ||"
          - "D3 Sar (signed arithmetic right shift)"
          - "D4 Ternary cond ? a : b"
          - "D5 Tuple in const positions"
          - "D6 type(uint).max/min, type(I).interfaceId"
          - "D7 Member: Lib.constant, Enum.Variant"
          - "D8 LitKind::Str + LitKind::Rational"
          - "D9 solc-equivalent convertType truncation/extension"
          - "D10 ExprKind::Array literal arrays"
        first_pr_target: |
          D1: wire comparison binops. Each match arm in eval_expr returns
          IntScalar::from_bool. Pair with tests/ui/typeck/const_eval_compare.sol
          covering signed/unsigned/edge-cases. ~250 LOC.
        references:
          - "solc libsolidity/analysis/ConstantEvaluator.cpp"
      typeck_cpp_todos:
        primary_files:
          - crates/sema/src/typeck/checker.rs
          - crates/sema/src/ty/mod.rs
          - crates/sema/src/ty/print.rs
        relevant_dirs:
          - tests/ui/typeck/
        oracle_commands:
          - "cargo nextest run -p solar-sema -- ui::typeck"
        slice_order:
          - "I1 super member-access typing for sub-contracts (checker.rs:1780)"
          - "I2 disallow super outside contract scope (checker.rs:2239)"
          - "I3 require(cond, MyError(...)) (checker.rs:287)"
          - "I4 variable-decl tuple LHS/RHS extras (checker.rs:1637)"
          - "I5 external fn-pointer return type in visit_ty (checker.rs:1904)"
          - "I6 mobile common-implicit type behavior (checker.rs:564)"
          - "I7 interfaceType implementation (ty/mod.rs:987)"
          - "I8 transient storage location resolution (ty/mod.rs:1294 unimplemented!())"
          - "I9 richIdentifier print parity (ty/print.rs:356)"
        first_pr_target: |
          I1: wire super member-access typing. Pair with
          tests/ui/typeck/super_member_access.sol. ~300-400 LOC.
      diagnostic_infra_gaps:
        primary_files:
          - crates/interface/src/diagnostics/emitter/human.rs
          - crates/interface/src/diagnostics/context.rs
          - crates/interface/src/diagnostics/message.rs
          - crates/interface/src/diagnostics/emitter/json.rs
        relevant_dirs:
          - tests/ui/cli/
        oracle_commands:
          - "cargo nextest run -p solar-interface"
        slice_order:
          - "E1 error-code -> docs URL mapping"
          - "E2 Suggestion / Applicability fix-it rendering"
          - "E3 stable error-code allocation registry"
          - "E4 JSON emitter parity for secondarySourceLocations"
          - "E5 SARIF emitter (new EmitterFormat::Sarif)"
          - "E6 implement remaining DiagnosticFormat variants (context.rs:193)"
          - "E7 implement remaining MessageKind rendering (human.rs:165)"
          - "E8 diagnostic deduplication"
          - "E9 multiline annotation primitive (replace human.rs:456 hack)"
          - "E10 inline-yul column-mapping for nested errors"
        first_pr_target: |
          E6 + E7: close the two unimplemented!() arms. Pair with focused
          unit tests in solar-interface and a UI fixture demonstrating the
          new render path. ~350 LOC.
      yul_typeck_tail:
        primary_files:
          - crates/sema/src/typeck/checker/yul.rs
          - crates/sema/src/typeck/checker.rs
          - crates/sema/src/ast_lowering/resolve.rs
        relevant_dirs:
          - tests/ui/yul_lowering/
        oracle_commands:
          - "cargo nextest run -p solar-sema -- ui::yul_lowering"
        slice_order:
          - "H1 switch default-required exhaustiveness"
          - "H2 reachability after revert/return/stop"
          - "H3 verbatim_*i_*o builtin signature exhaustive coverage"
          - "H4 nested-function scoping rules (no captures)"
          - "H5 label-style identifier rejection"
          - "H6 memory-safe mload/mstore alignment checks"
          - "H7 const-folding for numeric/bool literals"
          - "H8 object/code/data sections (parser already supports; typeck stub)"
          - "H9 external fn-pointer return-type check in visit_ty (checker.rs:1904)"
      standard_json_output_emitter_missing:
        primary_files:
          - crates/cli/src/lib.rs
          - crates/interface/src/diagnostics/emitter/json.rs
          - crates/sema/src/emit.rs
        relevant_dirs:
          - tests/ui/cli/standard_json/
        oracle_commands:
          - "cargo nextest run -p solar-compiler -- ui::cli::standard_json"
        slice_order:
          - "G1 outputSelection glob matching (settings.outputSelection)"
          - "G2 settings.remappings"
          - "G3 settings.libraries"
          - "G4 settings.optimizer + settings.evmVersion (recorded; codegen-side activates later)"
          - "G5 errors[] secondarySourceLocations"
          - "G6 sources[].id stability"
          - "G7 contracts[].evm subobject stub"
          - "G8 outputSelection wildcards (`*` and `*` per file)"
      bytecode_equivalence_harness:
        primary_files:
          - "tools/equiv-tester/Cargo.toml (NEW)"
          - "tools/equiv-tester/src/main.rs (NEW)"
          - "tests/codegen-equivalence/ (NEW)"
        relevant_dirs:
          - tools/
        oracle_commands:
          - "cargo run -p equiv-tester -- tests/codegen-equivalence/"
        slice_order:
          - "M1 tools/equiv-tester crate that runs solar+solc on a corpus and diffs canonical outputs"
          - "M2 curate ~20 canonical contracts under tests/codegen-equivalence/"
          - "M3 per-test ABI-diff (today)"
          - "M4 per-test bytecode-diff (after codegen-mir-split MIR+emit lands)"
          - "M5 per-test gas snapshot via revm (gated on bytecode availability)"
          - "M6 CI integration with skip-on-no-codegen"
      performance_leadership_unmeasured:
        primary_files:
          - benches/
          - "scripts/perf/sota-report.py (NEW)"
        relevant_dirs:
          - benches/
        oracle_commands:
          - "cargo codspeed build && cargo codspeed run"
          - "cargo bench -p solar-bench --bench iai"
          - "scripts/perf/sota-report.py --vs solc 0.8.31 --corpus performance-corpus"
        slice_order:
          - "P1 SOTA report scaffold: per-stage Solar-vs-solc-0.8.31 timing on pinned corpus"
          - "P2 frontend hot paths: lexer cursor / tokenization / parser advancement / string handling / source map"
          - "P3 sema hot paths: symbol interning / arena locality / resolver maps / type checker caches"
          - "P4 cold-build wall-clock target: 5x faster than solc 0.8.31 on full forge-std build"
          - "P5 typeck wall-clock target: 3x faster than solc -Ztypeck on syntaxTests/types/"
          - "P6 warm-rebuild target: 10x faster than solc on a one-leaf-edit smoke (depends on incremental compilation)"
        first_pr_target: |
          P1: build the SOTA report harness. scripts/perf/sota-report.py runs
          solar + solc on a pinned 5-contract corpus and emits a per-stage
          timing table (lex / parse / resolve / typeck / emit). Solar entries
          are best-of-N with environment metadata; solc entries are baseline
          for comparison. Drop into .pads-artifacts/sota.json. ~250 LOC.
      incremental_compilation_missing:
        primary_files:
          - "crates/sema/src/incremental.rs (NEW)"
          - "crates/sema/src/queries/ (NEW)"
        relevant_dirs:
          - crates/sema/
        oracle_commands:
          - "cargo nextest run -p solar-sema -- incremental"
        slice_order:
          - "INC1 design RFC behind --unstable-incremental flag"
          - "INC2 query trait + intern table + dependency tracking shell"
          - "INC3 parser query (file -> AST)"
          - "INC4 resolver query (AST -> resolved scope)"
          - "INC5 typeck query"
          - "INC6 invalidation + warm-rebuild benchmark"
        first_pr_target: |
          INC1: write the design RFC under research/incremental/RFC.md and
          add a feature flag `--unstable-incremental` that gates the entire
          subsystem. No production code yet; this PR is the design boundary.
      gas_aware_codegen_missing:
        primary_files:
          - "crates/codegen/src/passes/ (NEW; depends on codegen-mir-split landing)"
        oracle_commands:
          - "cargo nextest run -p solar-codegen -- gas_pass"
        slice_order:
          - "GAS1 storage-slot packing pass (vs solc ABIv2 layout)"
          - "GAS2 dead-store elimination on storage writes"
          - "GAS3 jump threading"
          - "GAS4 peephole patterns (push0/iszero/dup peepholes)"
          - "GAS5 stack-rematerialization vs spill heuristic"
        notes: "Each pass requires the bytecode-equivalence harness AND the codegen-mir-split campaign to be live."
      eof_support_missing:
        primary_files:
          - "crates/sema/src/eof/ (NEW)"
          - "crates/codegen/src/eof/ (NEW; depends on codegen-mir-split)"
        oracle_commands:
          - "cargo nextest run -p solar-sema -- eof"
        slice_order:
          - "EOF1 EOF section validation in parser/sema (no codegen yet)"
          - "EOF2 EOF-only constructs in typeck (rjump/rjumpi/rjumpv targets)"
          - "EOF3 EOF section emit in codegen (after codegen-mir-split)"
          - "EOF4 --evmVersion=osaka EOF mode"
      formal_verification_missing:
        primary_files:
          - "research/formal/ (NEW)"
          - "tools/lowering-validator/ (NEW)"
        oracle_commands:
          - "cargo run -p lowering-validator -- --rule storage-pack"
        slice_order:
          - "FV1 SMT harness skeleton (z3 or cvc5 via Rust binding)"
          - "FV2 first validated lowering rule + counterexample fixture"
          - "FV3 metamorphic relations on optimizer passes"
      continuous_fuzzing_missing:
        primary_files:
          - "fuzz/Cargo.toml (extend)"
          - "scripts/fuzz/continuous-runner.sh (NEW)"
          - "tests/regressions/ (NEW)"
        oracle_commands:
          - "cargo +nightly fuzz run differential-vs-solc -- -runs=10000"
        slice_order:
          - "FZ1 differential fuzzer scaffold (Solar + solc on same input -> diff)"
          - "FZ2 corpus minimization + dedup"
          - "FZ3 daily/hourly cron integration; auto-PR for new minimized regressions"
          - "FZ4 metamorphic transformations (constant folding, dead branches)"
      language_extensions_research:
        primary_files:
          - "research/extensions/ (NEW)"
        oracle_commands:
          - "cargo run -- --unstable-feature=<name> file.sol"
        notes: "Each extension lands behind --unstable-feature=<name> with a research/extensions/<name>/RFC.md, fixtures, and an explicit non-default flag. Promotion to default requires upstream maintainer policy change AND parity coverage on the existing 0.8.x surface."

    # Note (2026-05-18): a `first_pr_candidates` block was intentionally
    # REMOVED. PADS already deleted `starter_tasks` back on 2026-05-09
    # because the planner echoed them as the entire kickoff TASK_GRAPH,
    # collapsing every run into the same green-field scaffolding work.
    # A fixed list of "first PRs" has the same local-maxima failure mode:
    # five planners run, five identical PRs queue, none merge cleanly.
    #
    # The planner now grounds itself in `known_gaps` + `track_files`
    # + live workspace/issue/CI state and composes its own task each
    # run. `track_files` gives it real file paths so it does not
    # hallucinate. It is grounding, not prescription.
    continuation_rules:
      - "After a measurement artifact lands, choose implementation work from the largest newly measured failing family with owner files and a cheapest oracle."
      - "After a Standard JSON front-door PR lands, advance diagnostics/source identity before artifact-output parity."
      - "After a review rejects a patch for missing fixture IDs or proof, create a measurement task unless the fixture family is already known."
      - "After upstream moves in an owned lane, refresh only that lane's evidence before dispatching related work."
      - "Do not replay calibration slices once they have produced their unlock artifact; continue from live evidence."
      - "Keep editor-surface and backend-reference work as monitoring/spec/test-harness work until their unfreeze criteria and equivalence gates are met."
      - "When a feature family is parser-aware but not measured, dispatch fixture/oracle work before semantic or codegen implementation."
    pr_proof_rules:
      - "Semantic PRs need at least one non-format proof or an explicit typed reason why the task is measurement-only."
      - "Formatter evidence is a checkpoint, not a compatibility proof."
      - "Corpus PRs must name fixture IDs, corpus path family, solc version, and pass/fail/unsupported delta."
      - "Standard JSON PRs must name supported fields and unsupported outputs."
      - "Branch-mining PRs must name upstream commit(s), omitted work, conflicts, and local proof boundary."
      - "Foundry-adjacent PRs must name the pillar they touch: process contract, JSON I/O, source maps, artifact consumers, cache identity, or runtime/codegen."
      - "Hardfork or opcode PRs must name `evmVersion` and the solc/runtime evidence used."

non_goals:
  - SMTChecker parity unless separately approved.
  - Legacy Solidity language modes outside the declared 0.8.x compatibility surface.
  - Documentation-only churn that does not unlock implementation or verification.
  - Bytecode, runtime, optimizer, or performance claims ahead of the oracles required to prove them.
  - First-wave LSP, formatter, doc-generator, rename/refactor, or editor-extension product work before the editor-surface unfreeze criteria are met.
  - Stable public Rust library API commitments beyond upstream's binary-semver policy.
---
# Solar Autonomous Compiler Campaign

This file is the kickoff brief and operating constitution for autonomous work on `arjunblj/solar`. It is intentionally long. It is policy, project context, and the senior-engineer briefing a strong team would want before picking up Solar cold.

It is not a backlog. The structured sections above (`tracks`, `oracles`, `corpora`, `watchlist`, `extensions`) are project policy and verification grammar. They keep generated work honest. They do not enumerate which PRs to open. The `north_star_components` and `planning_guidance` blocks describe the shape of the work that should be planned. Everything below is project context a planner reads to write a real engineering plan.

The run should assume this mission when no explicit user prompt is supplied. The orchestrator should continuously discover, localize, rank, implement, review, publish, merge, and replenish work until the original mission is complete or remaining work is low value.

## Kickoff Operating Instructions

The first no-mission kickoff should not wait for a human to enumerate work. The mission is this file.

1. Bootstrap with `.pads/setup.sh` and record the exact tool versions.
2. Read this `PADS.md`, `AGENTS.md`, `.pads/README.md`, `.pads/spec.json`, `.pads/rules/*`, the fork diff, upstream issue/PR watchlist, and current CI status.
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

## Deep Research Synthesis

The May 16 research refresh sharpens the kickoff posture:

- Solar upstream intent is compatibility with Solidity `0.8.*`, not
  language divergence, pre-0.8 support, or production-readiness claims.
- Typechecker work is the clearest frontend parity contract: new or
  changed typechecker behavior needs fixtures, `-Ztypeck`, and pinned
  `solc 0.8.31` evidence.
- Standard JSON is the toolchain front door. Diagnostic JSON, direct
  file compilation, and `solar $(forge re)` are useful smoke tests but
  do not prove the solc process contract Foundry and Hardhat consume.
- LSP, formatter, doc-generator, rename/refactor, and editor-extension
  work are reference-only until upstream LSP lifecycle/symbol-table work
  is merged and frontend diagnostics are strong enough for editor use.
- Backend/codegen is required for a full Solar-as-solc replacement, but
  upstream `feat/codegen-mir` remains draft source material. Extract
  dependency-ordered specs, tests, and harness slices; do not merge the
  branch wholesale or claim correctness from known-red runtime CI.
- Performance work starts with measurement substrate and phase reports.
  Parser speed claims need parser correctness gates; typechecker speed
  claims need a real typechecker corpus; codegen/runtime speed claims
  need bytecode/runtime oracles.

High-risk feature families should bias planning toward fixtures and
oracles before implementation: ABI encoding/decoding, selectors, custom
errors, storage layout, transient storage, UDVTs, Yul/inline assembly,
constructors, immutables, metadata, library linking, source maps,
inheritance, receive/fallback, `try/catch`, and hardfork-gated opcodes.
Parser or symbol awareness is not support.

The compact rule files under `.pads/rules/` are the ingestion surface for
this synthesis. They are decision boundaries, not generated backlog:
`upstream-map.md`, `feature-matrix.md`, `foundry-readiness.md`, and
`performance.md` tell a harness when to proceed, when to measure first,
and when to stop with a blocker.

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

Foundry integration is product-critical. Solar exists to improve Solidity developer feedback loops, and Foundry is where most active Solidity developers feel the compiler. "Foundry support" is much more than running `solar $(forge remappings) file.sol`. The bar is: a real Foundry project's `forge build`, `forge test`, and `forge coverage` work end-to-end with Solar as the compiler, and produce artifacts that downstream tooling (cast, anvil, deploy scripts, indexers, etherscan verification) consumes the same way solc artifacts are consumed.

### The four contracts that have to hold

1. **Process contract** — Foundry consumes a solc-shaped subprocess. CLI flags (`--standard-json`, `--combined-json`, `--bin`, `--bin-runtime`, `--abi`, `--ast-compact-json`, `--metadata`, `--storage-layout`, `--allow-paths`, `--include-path`, `--base-path`, `--evm-version`, `--optimize`, `--optimize-runs`, `--via-ir`, `--no-cbor-metadata`, `--metadata-hash`, `--metadata-literal`), exit codes, stdout/stderr separation, error JSON shape, and version string format must match. `solar --version` should be parseable by Foundry's compiler-detection code as a Solidity-compatible compiler. **Mismatched stderr text alone can break Foundry's error parser** even when the compile itself succeeds.

2. **JSON I/O contract** — Standard JSON input/output is the integration front door. Inputs: `language` (must be `Solidity`), `sources` (with optional `urls`/`content`/`keccak256`), `settings.optimizer`, `settings.evmVersion`, `settings.viaIR`, `settings.metadata.bytecodeHash`, `settings.metadata.appendCBOR`, `settings.libraries`, `settings.remappings`, `settings.outputSelection` (with glob `*` semantics for files and contracts). Outputs: `errors[]` (component, severity, errorCode, message, formattedMessage, sourceLocation, secondarySourceLocations), `sources[].id`, `sources[].ast`, `contracts[][].abi`, `contracts[][].metadata`, `contracts[][].userdoc`, `contracts[][].devdoc`, `contracts[][].storageLayout`, `contracts[][].evm.bytecode.{object,opcodes,sourceMap,linkReferences,immutableReferences,generatedSources}`, `contracts[][].evm.deployedBytecode.{...}`, `contracts[][].evm.methodIdentifiers`, `contracts[][].evm.gasEstimates`, `contracts[][].ir`, `contracts[][].irOptimized`. Each output field that a Foundry consumer reads must match solc byte-for-byte (modulo declared metadata-strip normalization).

3. **Source identity contract** — `sourceName` (used as the artifact key by Foundry and Hardhat both) is computed from base path + include paths + remappings + allow paths + auto-detected libs. The same source file must produce the same `sourceName` under Solar as under solc with the same flags, otherwise downstream artifact lookups fail. `metadata.sources[<sourceName>].keccak256` and the metadata IPFS/swarm hash that ends up in the deployed CBOR section must match for verification flows (Etherscan, Sourcify, Foundry's `forge verify-contract`).

4. **Runtime contract** — once Solar emits bytecode, deployed behavior must match solc on the same input. Compare on `creation_bytecode` (metadata-stripped), `runtime_bytecode` (metadata-stripped), `link_refs`, `immutable_refs`, `return_data`, `revert_data`, `logs`, storage slot writes, created child contracts, and gas class (within a small tolerance until gas-aware codegen passes are explicitly enabled).

### Mixed-compilation harness (the strongest near-term oracle)

`FOUNDRY_SOLC=solar` is a smoke test, not the proof. The stronger target is **mixed compilation**: tests + script files + helper libraries compile with pinned `solc 0.8.31`, the contract under test compiles with Solar, and both sets are linked + executed in the same EVM (revm, Anvil, or `forge test --mt`). Behavior diffs surface real Solar-vs-solc semantic gaps without needing Solar's full backend to be production-ready, because Solar only has to compile the contract whose behavior we're testing — solc handles everything else. This is the dominant oracle for the codegen-mir-split campaign before bytecode-equivalence is fully wired.

Mixed-compilation harness should ship as `tools/mixed-compile/` with: a runner that takes a Foundry fixture, classifies each `.sol` as test-side or contract-under-test, dispatches to the right compiler, runs `forge build` then `forge test`, and produces a JSON report comparing pass/fail/revert reasons on each test case.

### Real-codebase corpus (the maximum-coverage push)

Run Solar against actively-deployed production codebases. These are the codebases real auditors and projects use as gold standards; pass rate against them is the measurable proxy for "Foundry users won't hit a wall."

Mandatory corpus (pin a specific commit per project; refresh quarterly):

- **OpenZeppelin Contracts** — primary corpus for ERC-20, ERC-721, ERC-1155, AccessControl, Governor, ERC4626, proxies, upgradeability. Massive surface for inheritance, overrides, modifiers, libraries, custom errors. **>1000 contracts**, deep typeck pressure.
- **Solady** — high-performance reimplementations of OpenZeppelin patterns. Heavy inline assembly, gas-tuned optimizations, exotic opcodes. Stresses Yul lowering and storage-layout edge cases.
- **Solmate** — minimalist primitive library; stresses generic arithmetic and library-via-using semantics.
- **PRBMath** — fixed-point math library; stresses fixed/ufixed type checking, generic multiplication/division semantics, overflow semantics, branchless arithmetic.
- **Uniswap v3 / v4 core + periphery** — production AMM; stresses storage-slot packing, callbacks, hook architecture, deep call graphs, library-heavy math, optimizer interactions.
- **Seaport** — complex order/match logic; stresses calldata-heavy ABI encoding/decoding, errors with payloads, custom-error selector emission, signature recovery.
- **Aave v3** — money market; stresses upgradeable proxy patterns, oracle integration, state-mutability inference under modifier composition.
- **Compound v3** — minimal money market; stresses immutable references, single-deployment patterns.
- **Chainlink contracts** — oracle library; stresses interface composition.
- **ENS** — name resolution; stresses recursive type resolution.
- **forge-std** — testing/cheatcode interfaces. Must compile cleanly because every downstream Foundry user pulls it.
- **Optimism** monorepo (subset) — production rollup contracts; stresses precompile awareness, EVM-version edge cases.

For each project: pin a commit, capture `forge config --json` and `forge build --build-info` once with solc 0.8.31, store the build-info as the golden artifact, and run the comparison loop continuously.

### Fast feedback loop (the iteration discipline)

The harness should run a short Foundry oracle loop continuously — ideally faster than a worker can spawn:

1. **Pick a small Foundry fixture** from `testdata/foundry-fixtures/minimal/` (Counter.sol-style, single contract, pinned solc 0.8.31, optimizer off, evmVersion cancun). Sub-second compile.
2. **Compile with solc + Solar separately**; capture Standard JSON output, build-info, artifacts.
3. **Diff** the JSON output field-by-field against the golden. The diff schema must be machine-readable: `{ field_path, expected, actual, mismatch_class }`.
4. **Promote the mismatch** into a regression fixture under `tests/regressions/foundry/<project>/<contract>.sol` if it's reproducible and minimizable.
5. **Run forge test in the mixed-compile harness** for any contract whose JSON diff is clean; if the test fails on Solar but passes on solc, the runtime mismatch is a higher-priority artifact than any JSON diff.

The full corpus runs nightly (cron via `pnpm pads loop` or a dedicated GitHub Actions matrix on the fork). The minimal fixture runs on every PR. Coverage scales by adding fixtures, never by relaxing diff strictness.

### Build-info replay (the proof Foundry users will accept)

`forge build --build-info --build-info-path out/build-info` writes the canonical Standard JSON input + output for a project. **Replaying that exact input through Solar and producing the same Standard JSON output** is the strongest pre-runtime parity claim. It proves Solar correctly handles the exact paths/remappings/settings that Foundry generates for a real project, not a synthetic input we wrote ourselves.

The replay tool (`tools/build-info-replay/`) reads a build-info file, extracts the `input` field, pipes it through `solar --standard-json`, and diffs the output against the build-info's `output` field on the fields we currently emit. Skip diffs are recorded as `unsupported_field`, hard mismatches as `regression`. Every meaningful frontend lane closes by adding to the supported-field set or shrinking the unsupported-field set on the build-info corpus.

### Hardhat parity (secondary but tracked)

Hardhat consumes the same Standard JSON contract but writes artifacts in a different on-disk layout and uses `inputSourceName` separately from `sourceName`. After Foundry parity is solid, port the build-info replay tool to read Hardhat artifacts and prove the same field set works there too.

### What "Foundry support shipped" looks like

- `forge build` and `forge test` succeed on the mandatory corpus with Solar configured as the compiler, with paired solc-baseline behavior.
- `forge verify-contract` works against Etherscan/Sourcify on a Solar-compiled contract (metadata + sourceName parity).
- The mixed-compile harness reports zero behavioral regressions on the mandatory corpus.
- Build-info replay reports zero hard mismatches on the supported-field set across the mandatory corpus.
- Documentation under `docs/foundry-integration.md` tells a Foundry user exactly which `solc_version` / `via_ir` / `evm_version` combinations Solar supports today.

Stop short of claiming "drop-in solc replacement" until all four contracts hold for the mandatory corpus.

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
- `#726`: PGO/BOLT experiment; reproduce only on pinned correctness-gated corpora before considering defaults.
- `#743/#744/#755/#758`: narrow current upstream fixes.
- `#401/#417/#418/#419/#420/#421/#416`: LSP/editor-surface work; reference-only until merged/released and frontend oracles are stronger.
- `#567`: NatSpec lowering; artifact compatibility context, not standalone docs product.
- `foundry-rs/foundry#9317/#11307/#11652/#12721/#10965`: Foundry Solar adoption around AST/context/lint/backtrace/flatten.
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
- first-wave LSP/editor/formatter/doc-generator work before upstream unfreeze criteria and frontend proof strength exist
- opcode, hardfork, storage-layout, ABI, source-map, metadata, library-linking, or immutable-reference work without the matching solc/runtime artifact oracle
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

## Self-Replenishing Work Loop

The harness should not stop when the initial 12 campaigns wrap. The completion contract is far larger than the initial lane catalog. As campaigns close, the planner discovers new work via these heuristics — running them on every kickoff is mandatory; running them after every merged PR in a campaign is encouraged.

### Discovery heuristics (run continuously)

1. **`unimplemented!` / `todo!` / `FIXME` sweep.** After each session, run `rg -n 'unimplemented!|todo!|^[[:space:]]*//.*FIXME|^[[:space:]]*//.*TODO' crates/sema crates/parse crates/ast crates/interface crates/codegen 2>/dev/null` and cross-reference against the open task list. Each new match that is NOT already represented by a task becomes a candidate lane. The planner ranks candidates by surrounding code complexity (longer surrounding function = more leverage) and adjacency to recently-merged work.

2. **Solc-equivalence delta sweep.** After each merged PR, run `TESTER_MODE=solc-solidity-typeck cargo nextest run -p solar-compiler --test tests` and record the pass/fail/unsupported count delta against the previous baseline. If new failures appear (regressions), they jump the queue. If newly passing tests appear, narrow the xfail set. The fixture-by-fixture failure list is the single biggest source of new lanes after the initial 12 campaigns.

3. **Upstream branch sweep.** Daily, run `gh api repos/paradigmxyz/solar/branches --jq '.[].name' | rg '^(dani|feat)/'` and diff against the previous day. New branches mean new in-flight work areas; if a branch shipped on upstream main, the relevant area is no longer actively defended and the harness can pursue adjacent slices. Branches that touch the same files as our open lanes become **collision-pause** signals; pause those lanes until the upstream branch lands.

4. **Solc syntaxTests mining.** After each typeck-area merge, scan `testdata/solidity/test/libsolidity/syntaxTests/` for directories where Solar's pass rate is below 90%. Each directory becomes a candidate campaign of its own (e.g., `viewPureChecker/`, `inheritance/`, `using-for/`, `userDefinedValueType/`). The harness is licensed to author new fixtures under `tests/ui/typeck/` that mirror solc's test cases as long as they're paired with production code that actually fixes the underlying invariant.

5. **Real-codebase regression sweep.** Nightly, run the Foundry mandatory corpus and record build-info replay diffs + mixed-compile runtime diffs. New diffs are new lanes. Old diffs that disappear are progress signals.

6. **Codegen MIR backlog drain.** Until `feat/codegen-mir` is fully replayed, the harness has a guaranteed steady stream of work: read the next slice from upstream, re-author it on fork main, ship as a single PR. The harness should always have at least one open codegen-replay PR while this campaign is active.

7. **Continuous fuzzer findings.** When the differential fuzzer surfaces a minimized regression, it auto-creates a task with the fixture pre-staged in `tests/regressions/`. The harness picks it up in the next dispatch wave.

8. **Performance leadership backlog.** As correctness oracles for a stage land green, that stage becomes eligible for performance work. The SOTA report identifies the slowest stage relative to solc; the harness picks the slowest stage with a green correctness oracle.

### Replenishment cadence

- **Every kickoff** runs heuristics 1, 2, 3, 4, 7, 8 and emits new tasks for the result set.
- **Every merged PR in a campaign** triggers heuristic 1 + 2 + the campaign-specific auto-discovery hint (e.g., closing one Const-Eval lane re-runs `rg 'UnsupportedBinaryOp|// hir::ExprKind::' crates/sema/src/eval.rs`; closing one View/Pure lane sweeps `unimplemented!()` in `crates/sema/src/typeck/`).
- **Nightly** runs heuristic 5 against the full Foundry mandatory corpus.
- **On demand** when a campaign visibly stalls (no merged PR in N days for that track), the planner runs heuristics 1+2+4 specifically for that track's scope.

### Auto-discovery is not a replacement for the catalog

The catalog (`tracks` + `extensions.solar.track_files`) is grounding. Auto-discovery is a feed of new candidates that the planner scores against the catalog. A discovered candidate that does not fit any existing track is parked into `extensions.solar.parked_candidates` (a wiki entry, not PADS.md edits) for the operator to triage. The harness must not invent new tracks; it can only deepen existing ones.

### Stop conditions

The autonomous loop should run until ALL of these hold:

- All `not_shipped` items in `extensions.solar.current_state` are shipped or formally re-classified as out-of-scope.
- Every `completion_contract` line has paired evidence in the wiki.
- The Foundry mandatory corpus runs clean end-to-end on three consecutive nightly runs.
- The differential fuzzer has produced no new hard regressions for 7 consecutive days.
- The SOTA performance report meets the floor targets for cold/warm/typeck speedups vs solc.

Until then, "we're done" is wrong; keep replenishing.

## Multi-Week Autonomy Posture

The harness is configured to run unattended for multi-week campaigns. The operator should expect to be SHOCKED by what it accomplishes in 2-4 weeks; that requires the harness to be set up correctly so it does not silently degrade.

### Posture rules (apply across all campaigns)

- **No mission text from the CLI**; PADS.md owns intent. Passing `--mission` to `pads kickoff` overrides PADS.md and biases workers, exactly the failure mode this section is designed to avoid.
- **The loop is `pnpm pads loop arjunblj/solar --host <host> --budget 1000 --interval-s 300 --initial-wait-s 600 --out-dir tmp/canaries --max-iterations 0`** (max-iterations=0 means unbounded; the loop ends when the operator says so or the stop conditions above hold).
- **Canary cadence target**: a fresh kickoff every ~30 minutes when the previous canary is GREEN; immediate triage and replan on RED. The trajectory log (`tmp/canaries/index.md`) is the single source of truth for "is the harness making progress."
- **Failure modes that auto-pause the loop** (defined by the harness, listed here for operator awareness): RED grade twice in a row with the same top finding; sandbox prewarm circuit open; >50% of dispatched workers blocked on the same path; cargo build broken on fork main for >2 hours.

### Image expectations

The sandbox image is the substrate that determines how much MEATY work the harness can do per hour. Operator must keep these in good shape; PADS.md records them so kickoff can verify and refuse to run if they fail.

- **rustc 1.95.0** as default toolchain (matches workspace MSRV).
- **rustc nightly** for `cargo +nightly fmt` (rustfmt.toml uses unstable features).
- **cargo-nextest, typos, cargo-hack, cargo-deny, cargo-codspeed, cargo-docs-rs** preinstalled.
- **forge + solc 0.8.31** preinstalled with `solc-select use 0.8.31` configured.
- **python3 + pip + PyYAML** preinstalled (used by `scripts/pads/baseline-ledger.py` and `scripts/pads/spec-sync.py` if present).
- **Pre-warm cache**: `cargo fetch --locked && cargo nextest run --no-run --workspace --locked` baked into the image so the first worker session does not pay 6-15 minutes of cold compile.
- **Per-sandbox sizing**: 8 vCPU / 8 GB RAM minimum; bump for the codegen-mir-split campaign which needs to compile MIR + EVM emit modules. Disk: >=20 GiB to fit `target/` for parallel workers.
- **Concurrent worker slots**: target 6+ parallel workers; verified end-to-end against the sandbox prewarm circuit.

### Drift detection

Solar's upstream moves daily. The harness must detect and react to drift without operator steering:

- **MSRV drift**: if `rust-toolchain.toml` on fork ever falls behind the workspace `Cargo.toml` `rust-version`, the next kickoff fails fast with a typed blocker.
- **Cargo.lock drift**: if `cargo update` produces a >10% lockfile churn, hold for operator review (workspace-wide dep updates are a `must_pause_for_approval` class).
- **Upstream main drift**: if upstream main moves >50 commits ahead of the most recent fork sync point, the harness emits a "sync recommended" wiki note (does not auto-sync; that's an operator decision).
- **Test corpus drift**: if `testdata/solidity` submodule HEAD changes (operator updates the pinned solc test corpus), the next kickoff re-baselines the typeck/syntax counters.

### Quality watchdog

The harness must KEEP quality high even when shipping fast. The watchdog runs on every accepted PR before it lands on fork main:

- The PR must touch production code on its substantive lane (not tester-only or scaffolding-only) — the existing `coerceNonProductionPatchOnSubstantiveLane` rule.
- The PR body must contain the four required sections (behavior diff, failing oracle, passing oracle, named corpus/fixtures).
- The diff must not exceed `permitted_side_effects.max_files_per_pr: 30` or `max_crates_per_pr: 4` without explicit operator override.
- The CI gate (typos, fmt, clippy, nextest workspace, cargo build, cargo doctest) must be green.
- No filler words in the PR body (banned list in `pr_rubric.ban_filler_words`).

If any check fails, the PR auto-stays in draft and the harness opens a follow-up task to fix the issue rather than merging the broken work.

## Beyond-Solc Frontier

Once parity is in hand, the project's reason to exist is to LEAD where solc cannot. The frontier campaigns below are explicitly out-of-scope for "feature parity"; they are about why a Solidity developer should choose Solar over solc once parity is no longer a question. Each frontier campaign requires correctness oracles to land first and runs behind feature flags until graduation criteria are met.

### Frontier campaigns

- **Incremental compilation (Salsa-style query graph).** Solc recompiles eagerly; Solar can ship a query-based architecture (parser query, resolver query, typeck query, codegen query) with explicit invalidation tracking. The win is a 10x+ warm-rebuild speedup on real projects (one-leaf-edit smoke). Phases: design RFC behind `--unstable-incremental` -> query trait + intern table -> wire parser -> wire resolver -> wire typeck -> measure warm rebuild vs cold. Each phase ships clean-vs-incremental equivalence tests as a hard invariant; clean and incremental must produce the same artifacts on the same input or the PR is rejected.

- **Gas-aware codegen passes (post-MIR-split).** Solc's codegen leaves observable gas on the table. Once Solar has the MIR + EVM emit infrastructure, ship passes that solc lacks: storage-slot packing tighter than ABIv2 layout, dead-store elimination on storage writes, jump threading, peephole patterns (push0/iszero/dup), stack-rematerialization vs spill heuristic. Each pass requires the bytecode-equivalence harness AND the codegen-mir-split campaign to be live; gates on a per-contract gas-decrease fixture + paired runtime-equivalence proof.

- **EOF (EVM Object Format) support.** Post-Cancun EVM Object Format is not in solc 0.8.31. Solar can ship EOF section validation in parser/sema, EOF-only typeck constructs (rjump/rjumpi/rjumpv targets), EOF section emit in codegen, and `--evmVersion=osaka` mode. Validation lands first (independent of codegen); emit lands after codegen-mir-split delivers MIR+emit infrastructure.

- **Formal-rewrite-validated optimizations.** SMT-backed validation of lowering rules: each new optimizer pass ships with an SMT artifact (z3 or cvc5 via Rust binding) that proves the rewrite preserves semantics on the abstract domain, plus a counterexample fixture corpus. Metamorphic relations on optimizer passes (constant folding, dead branch elimination) catch regressions automatically.

- **Continuous differential fuzzing.** Always-on differential fuzzer comparing Solar and solc on generated inputs. Auto-minimization to deterministic regression fixtures under `tests/regressions/`. Runs hourly (or on a dedicated worker continuously); each new minimized regression auto-creates a task in the harness queue with the fixture pre-staged.

- **Richer diagnostics than solc.** rustc-style fix-it suggestions with `Applicability` levels, stable error-code registry with documentation URLs, SARIF output for CI integrations, multiline annotations using annotate-snippets v0.11 primitives, inline-yul column-mapping for nested errors. Together these graduate Solar's diagnostics from "as good as solc" to "noticeably better than solc."

- **Linter (solar-lint) with composable lint passes.** Solc has no lint pass; tools like solhint and slither sit out-of-tree with reduced semantic awareness. Solar can ship lints that consume the typed HIR and CFG: unused state vars, builtin shadowing, empty contract body, i++ vs ++i, missing docstring on public surface, unchecked external-call return value, unbounded loops, gas-suspicious patterns. Each lint is its own PR; the framework ships first.

- **Language extension experiments (behind `--unstable-feature=<name>` flags).** Speculative additions that solc cannot easily ship because of stability commitments: compile-time generics on functions, structural type narrowing, `constexpr`-style compile-time evaluation, gas-aware diagnostics in source. Each extension lands behind a flag with a `research/extensions/<name>/RFC.md`, fixtures, and an explicit non-default. Promotion to default requires upstream maintainer policy change AND parity coverage on existing 0.8.x surface.

### Graduation criteria from frontier to default

A frontier feature graduates from `--unstable-feature=<name>` to default-on only when:

1. Parity coverage on existing 0.8.x surface is unchanged or improved.
2. The frontier feature has paired correctness oracle (clean-vs-frontier equivalence on the no-feature-used path).
3. The frontier feature has paired performance oracle showing it does not regress the no-feature-used path.
4. The Foundry mandatory corpus is green with the frontier feature enabled.
5. Operator approval is recorded.

Until then, frontier features stay opt-in.

## Performance Leadership Targets

Performance leadership is a first-class project goal, not a nice-to-have. Concrete targets, all measured on the pinned performance-corpus against pinned solc 0.8.31 in the same environment.

### Targets (floor; the project should beat them, not match)

| Stage | Target | Rationale |
| --- | --- | --- |
| Cold build (full forge-std + OpenZeppelin compile) | >=5x faster than solc | Justifies "drop solc, use Solar" for CI builds. |
| Warm rebuild (one-leaf-edit smoke) | >=10x faster than solc | Justifies the incremental compilation campaign; Solar's biggest near-term differentiator. |
| Typeck-only (`-Ztypeck` over syntaxTests/types/) | >=3x faster than solc -Ztypeck | Direct measurement of frontend speed. |
| Parser throughput | >=50 MiB/s/core on real Solidity source | Sets the floor for end-to-end speed; correlated with cold build. |
| Memory peak (full project compile) | <=80% of solc's peak RSS | Solar must not trade memory for speed. |
| Cold start latency (`solar --version` to first compile output) | <=200ms | Process-startup matters in tight CI loops. |

### Non-negotiable performance discipline

- **No performance PR without a paired correctness PR or oracle.** A speedup that breaks correctness is a regression, not progress.
- **Every performance PR includes**: baseline command + head command in the same environment, base SHA, head SHA, solc version, rustc version, target triple, CPU model, correctness oracle result for the optimized stage, primary metric with direction and noise policy, profile evidence naming the hot path changed, before/after numbers, statement of which corpora moved.
- **Microbenchmark-only wins are rejected.** A PR that improves a microbench but regresses a macro corpus is a regression.
- **Profile before optimizing.** A perf task without a hot path is a research task, not an implementation task.
- **Performance budget**: every campaign's done-condition includes a perf check that the campaign did not regress the SOTA report by more than 2% on any stage.

### Performance leadership requires the incremental campaign to land

The 10x warm-rebuild target is unachievable without the incremental compilation campaign. That makes incremental compilation a hard prerequisite for the warm-rebuild target, not a nice-to-have. Schedule the incremental campaign in week 2 once parser/sema correctness oracles are landing routinely.
