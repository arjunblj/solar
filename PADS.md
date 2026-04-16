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
    baseline_commit: 2140f36

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
      - open PRs within preset blast-radius caps
      - land research prototypes under research/
    permitted_side_effects:
      max_files_per_pr: 20
      max_crates_per_pr: 2
      max_wall_time_per_task_min: 60
      max_api_spend_per_task_usd: 5
    must_pause_for_approval:
      - .github/workflows/**
      - testdata/solidity/**
      - new crate, new IR, or new pass pipeline proposals
    shutdown_timer: { wall_clock_days: 7 }

  edit_rules:
    - Tier-0 is human-only. Requires atomic update of PADS.md + .pads/tier0.sha256.
    - Tier-1 edits need critic-agent review + human approval.
    - System memory (lessons, rejected ideas, rules, status) lives in the pads platform, not in this repo.

oracles:
  - { id: cargo.build,       kind: shell, tier: prerequisite, command: "cargo build --workspace" }
  - { id: cargo.nextest,     kind: shell, tier: gate,         command: "cargo nextest run --workspace" }
  - { id: cargo.uitest,      kind: shell, tier: gate,         command: "cargo uitest" }
  - { id: cargo.clippy,      kind: shell, tier: gate,         command: "cargo clippy --workspace --all-targets -- -D warnings" }
  - { id: cargo.fmt,         kind: shell, tier: gate,         command: "cargo +nightly fmt --all --check" }
  - { id: solc_syntax_tests, kind: shell, tier: gate,         command: "TESTER_MODE=solc-solidity cargo nextest run -p solar-tester" }
  - { id: solc_yul_tests,    kind: shell, tier: gate,         command: "TESTER_MODE=solc-yul cargo nextest run -p solar-tester" }
  - { id: codspeed_check,    kind: shell, tier: advisory,     command: "cargo codspeed build && cargo codspeed run" }

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

priority_order: [typeck, yul-hir, testing-infra, diagnostics, codegen-mir, bytecode-equivalence, performance, research-sota, fuzz-hardening, lsp]
---

# Solar

This is the soul of the Solar project. It tells you what we're building, why, how the codebase works, what the concrete next steps are for every track, and what the research frontier looks like. Read it as a comprehensive brief from an eng manager to a team of contractors who are smart but have zero Solar-specific context.

## The bet

Compiler work has **verifiable loops**. Unlike most software, a compiler has an unfakeable oracle: compile the same input with the reference compiler (solc) and the one under test (solar), diff the output. If they match, the change is correct. If they don't, the divergence is a concrete bug. This means an autonomous system can produce merge-worthy PRs over sustained runs — weeks, not hours — because every PR carries its own evidence.

The end state: a Paradigm reviewer looks at a Solar PR and merges it unchanged, because the oracle evidence is already in the body.

## Where Solar is today

Solar is ~39k LoC across 9 crates. The architecture is a near-carbon-copy of rustc's frontend: bumpalo arenas, `Gcx`/`GlobalCtxt`, `Interned<T>`, `IndexVec<Id, T>`, rustc-style diagnostics. The pipeline today is:

```
lexer → parser → AST → HIR → sema/typeck → diagnostics/ABI output
```

Codegen, Yul→HIR lowering, CFG, static analysis, and lint infrastructure are all future work. The `sema` crate has a `-Ztypeck` flag behind which the real type checker sits. The crate breakdown is roughly:

- `solar-parse` (~8.3k LoC) — lexer and parser
- `solar-ast` — AST definitions and visitors
- `solar-sema` (~13.5k LoC) — semantic analysis, type checking, HIR
- `solar-interface` (~8.6k LoC) — diagnostics and source management
- `solar-config`, `solar-cli`, `solar-macros` — supporting crates
- `tools/tester` — the corpus test runner (three modes: `Mode::Ui`, `Mode::SolcSolidity`, `Mode::SolcYul`)

**What's solid:** parser/lexer/AST fundamentals, diagnostics infrastructure, large parts of sema and typeck foundations, override checker and many conversion rules, UI test infrastructure, benchmark infrastructure.

**What's open:** broader `-Ztypeck` coverage, NatSpec lowering, Yul/inline-assembly lowering, LSP, codegen (not yet in this fork), bytecode equivalence harness, stronger differential testing.

## The oracle hierarchy

This is how you prove a change is correct. Lower tiers are cheaper and faster; higher tiers are more expensive but provide stronger evidence. Gate a change on the **lowest tier that proves it**.

| Tier | What | Cost | Signal |
|------|------|------|--------|
| 0 | `cargo fmt`, `typos` | <5s | cosmetic |
| 1 | `cargo check`, `clippy` | 10–120s | compiles + lints |
| 2 | `cargo nextest run` | 30–180s | unit test correctness |
| 3 | `cargo uitest` on owned fixtures | ~10s | diagnostics exactness |
| 3' | `cargo uitest` against `testdata/solidity/` submodule | ~1 min | diagnostics on unownable corpus |
| 4 | `solc_syntax_tests` — 3,551 error-code+span matches | 5–30 min | diagnostic coverage breadth |
| 5 | AST JSON diff, ABI diff, NatSpec diff vs solc | 1–2 min | frontend emission shape |
| 7 | Runtime bytecode diff solc vs solar | 1–10 min/contract | codegen correctness |
| 8 | `revm` differential execution | ~30 min | end-to-end runtime agreement |
| 9 | Grammar fuzz (Erwin, Solsmith, tree-crasher) | continuous | unknown-unknowns |
| 10 | Metamorphic invariants (rename, whitespace, literal equivalence, optimizer idempotence) | 10–60 min | universal invariants |
| 11 | `hevm` symbolic equivalence, Alive2-style pass proofs | minutes–hours | **proof** of equivalence |

A parser error-message tweak = tier 3 in 10s. A MIR pass = tiers 7+8+10+11. A typeck slice = tier 4.

## solc as the behavioral oracle

`testdata/solidity/` is a git-submodule of the argotorg/solidity fork at a pinned SHA. `tools/tester` runs it in three modes. PR #737 enabled `-Ztypeck` for ~1,593 of the ~3,551 solc syntaxTests. Every merged typeck PR in Solar's history cites a specific solc error code and often a specific line in `libsolidity/analysis/TypeChecker.cpp` by commit SHA. Follow that pattern.

Green Solar tests do not imply full solc parity. Parser-only corpus modes do not prove runtime correctness. The `TESTER_MODE=solc-solidity` run is primarily a parser/corpus oracle unless explicit `-Ztypeck` routing is enabled for the relevant semantic family.

## Relationship to upstream

`paradigmxyz/solar` is reference material — roadmap issues ([#1](https://github.com/paradigmxyz/solar/issues/1), [#615](https://github.com/paradigmxyz/solar/issues/615), [#687](https://github.com/paradigmxyz/solar/issues/687)), merged history, open PRs, docs and testing patterns. If a branch or PR isn't merged into **this fork**, it doesn't exist here. `feat/codegen-mir` (PR #693) is reference-only. The `E-easy` label is essentially drained (only #305, S-blocked); the real frontier is `E-medium`/`E-hard`.

## Track-by-track guide

### Typeck — close the TODOs (primary, highest leverage)

Issue [#615](https://github.com/paradigmxyz/solar/issues/615). The active human focus with the strongest built-in oracles.

**Concrete backlog** from `crates/sema/src/typeck/checker.rs`: L327 overload resolution, L335 "Did you mean...", L465 constant `expected.mobile`, L544/L615 custom operators, L1403 internal function pointer comparison, L1447 `super` disallow, plus solc line-number references at L713, L1219, L1421, L1583.

**Shape:** each TODO = 1 PR, ~200–800 LoC. Cite the solc error code + the exact line in `libsolidity/analysis/TypeChecker.cpp`. Ship with `cargo uitest` fixtures under `tests/ui/typeck/`. Oracle: `cargo nextest` + `cargo uitest` + `solc_syntax_tests` pass rate delta.

### Yul → HIR lowering (primary, parallel with typeck)

Issue [#415](https://github.com/paradigmxyz/solar/issues/415) (P-high, E-hard). Currently `hir::StmtKind::Assembly` is commented out at `crates/sema/src/hir/mod.rs` L1163–1165 — inline assembly silently disappears from HIR.

**Shape:** decompose by Yul statement kind, one PR each: `let`, `assign`, `if`, `for`, `switch`, `break`, `continue`, `leave`, `functionDef`, `expr`. Oracle: `cargo uitest` on new `tests/ui/yul-hir/` fixtures + `solc_yul_tests` pass rate. dipanshuhappy's draft [#652](https://github.com/paradigmxyz/solar/pull/652) has the open design questions.

### Testing infra (primary, continuous)

Port more solc syntaxTests under `-Ztypeck` (mirror PR [#737](https://github.com/paradigmxyz/solar/pull/737) per category: `nameAndTypeResolution/`, `abiEncoderV2/`, `inheritance/`, `modifiers/`, `structs/`, `immutable/`, etc.). Each PR: 5–15 new `tests/ui/typeck/<category>/*.sol` with `//~ ERROR: TypeError NNNN` annotations ported from solc's footer format. Oracle: `cargo uitest` + `solc_syntax_tests` pass rate tick.

The `tools/tester/src/solc/solidity.rs` skip-list is the public progress metric: every removed entry is a landed PR.

Erwin (`fuzz/run_erwin.sh`) already supports `--target solar`. Extend to run continuously; every divergence is a regression-test PR. Erwin's initial research found 26 bugs across solc/solang/solar.

### Diagnostics (supporting)

Port one solc error code per PR from `libsolidity/analysis/*.cpp` into Solar's `error_code!(NNNN)` with a matching UI fixture. AGENTS.md style rules apply: no trailing periods, backtick code references, `note`/`help`/`span_note` subdiagnostics. Use `sym::name` or `kw::Keyword` instead of `.as_str()`. Visitor pattern always calls `walk_*` continuation.

### MIR + codegen (draft — scaffolding phase)

Issue [#687](https://github.com/paradigmxyz/solar/issues/687) is a fully specified 11-sub-issue roadmap authored by gakonst. This fork does not yet contain `crates/codegen` or `crates/mir`. The dependency order is:

1. Bytecode-equivalence harness (#704) — unlocks the runtime oracle
2. Liveness (#694) → PhiElim (#695) → DCE (#700) → ConstFold (#701) → SCCP (#702) → CSE (#703)
3. Stack Model ≤16 (#696, CRITICAL) and Full Stack Scheduling (#697, blocked on #696) — escalate for human design review

Each issue body has pseudocode and "patterns from Venom/Sonatina" — good agent input. One pass = one PR, ~400–1200 LoC. Oracle ladder: cargo build → nextest → runtime bytecode diff → revm execution on OZ/Uniswap/Solady corpora. **Do not claim codegen progress without the runtime oracle.**

### Bytecode equivalence harness (draft — gating unlock for codegen)

Issue [#704](https://github.com/paradigmxyz/solar/issues/704). gakonst phrasing: "Foundry property tests + Anvil, compile with solc → bytecode A, compile with Solar → bytecode B, deploy both, assert same return values, state changes, reverts, gas (within tolerance)." Adapter crate in `tools/diff-harness/` using alloy + revm + foundry-compilers.

### Performance (supporting)

Protect Solar's front-end performance identity. Hot-path patterns must remain explicit: `sym::`, `kw::`, arenas, visitor discipline. Run `cargo bench -p solar-bench --bench criterion -- --quick` when hot paths change. CodSpeed + iai-callgrind run on every PR; auto-draft when delta exceeds -5%.

### LSP (draft — blocked on typeck)

Issue [#394](https://github.com/paradigmxyz/solar/issues/394) sub-issues: #416 flychecks, #417 lifecycle, #418 symbols, #419 autocomplete, #420 go-to-def, #421 inlay hints. This fork doesn't have `crates/lsp` yet. Not dispatched until typeck is stable enough.

### Fuzzing + hardening (supporting)

Expand malformed-input resilience and crash resistance. `fuzz/run_erwin.sh` is the main tool. Every divergence or crash is a regression fixture PR.

### Novel SOTA research (always-on, never blocked)

This track brings 2024–2026 compiler research into Solar as prototypes under `research/`. Ships on research branches; never blocks primary PRs. Doubles as the keep-alive filler when priority queues drain. Five streams:

**Aegraph-based Yul optimizer** — acyclic e-graph + sea-of-nodes-with-CFG + ISLE-like DSL + eager rewrites + cost-based extraction (Fallin 2026 Cranelift retrospective: aegraphs beat full equality saturation for rulesets in the hundreds). Crate at `research/solar-aegraph-yul/`.

**Alive2-for-EVM translation validation** — SMT-based bounded verification of every Yul rewrite rule (Lopes et al. PLDI 2021, Crocus/VanHattum ASPLOS 2024). Per-rule SMT discharge removes the test-coverage gap. Embarrassingly parallel — ideal for the orchestrator. Crate at `research/solar-yul-tv/`.

**EVM superoptimization** — EBSO/SuperStack unbounded SMT superoptimizer + Hydra-style generalization of missed peepholes into bitwidth-agnostic, SMT-verified rewrite rules (Nagele, Albert PLDI 2024, Regehr OOPSLA 2024). Each generalized rule is a tight, single-PR-sized unit. `research/solar-superopt/`.

**Continuous metamorphic fuzz** — CSmith/EMI lineage (Yang/Regehr PLDI 2011, Le/Su PLDI 2014) applied to Solidity via Erwin + Solsmith + tree-crasher. 17 metamorphic relations: rename-invariance, whitespace-invariance, hex/decimal literal equivalence, import-order invariance, optimizer idempotence, etc. Each is one PR. Infinite supply.

**LLM rewrite-rule synthesis** — Hydra-shaped (OOPSLA 2024): sample a missed peephole from the superopt or fuzz output, propose a bitwidth-agnostic rewrite rule via LLM, discharge via SMT (translation validation infra), if green add to the ISLE-DSL. This loop has infinite valid work and is the ultimate keep-alive filler.

## What counts as done

A task is done when:
- The change exists in this fork's default branch
- It has focused tests that prove the specific behavior changed
- The oracle evidence is in the PR body — not "CI passed" but "solc_syntax_tests: 2840/3551 → 2847/3551" or "cargo uitest: new error code 3726 fixture passing"
- A reviewer could merge it without asking follow-up questions

A task is NOT done when "it compiles" (tier 1), "tests pass" (unnamed tests), or codegen progress is claimed without a runtime differential.

## House style

Solar ships an in-tree `AGENTS.md` and `CLAUDE.md` authored by @onbjerg. Follow them exactly:
- Conventional Commits: `feat(sema): …`, `fix(parse): …`
- Diagnostic style: no trailing periods, backtick code refs, `note`/`help`/`span_note` subdiagnostics
- Use `sym::name` or `kw::Keyword` instead of `.as_str()`
- Visitor pattern: always call `walk_*` for child traversal
- Arena allocation for AST: no `Box<T>` outside arenas
- UI tests: `//~ ERROR:`/`WARN:`/`NOTE:`/`HELP:` annotations

## Rejected design decisions

These are recorded in the pads platform's memory. Do not re-derive them:
- **No salsa retrofit** — the current `cached!` memoizer is adequate; salsa is a multi-week cross-cutting change with unclear win (matklad 2026, @DaniPopes on #91).
- **No direct Venom adoption** — Solar's MIR should be shaped by Solidity's HIR, not Vyper's tree. Borrow patterns from Cranelift/Sonatina, not Venom.
- **No full equality saturation** in the Yul optimizer — Fallin 2026 is explicit: aegraphs + eager rewrites beat egglog at Solar's ruleset size.
- **No self-hosting before codegen** — Solar can't compile itself until codegen emits correct EVM bytecode. 2027+ aspiration.
- **No rebless of testdata/solidity/ snapshots** — that submodule is the oracle. Disagreements are divergences to document or bugs to fix.

## The wiki — your knowledge base

Deep domain context, research findings, synthesis documents, and accumulated lessons live in the **pads wiki** (the project's structured knowledge store, surfaced in the pads UI and queryable via the orchestrator API). The wiki is hierarchical and searchable — treat it as your long-term memory.

**Before starting any non-trivial task**, query the wiki for prior findings on the same topic. The wiki already contains research on: AI PR acceptance patterns and what makes maintainers merge, context engineering for long-running agents, Solar-specific contribution patterns, oracle design for compiler correctness, sandbox pipeline architecture, subagent coordination strategies, and more. These were produced by dedicated research passes and are higher-quality than what you'd get by re-deriving them from scratch. Use them.

**After completing a task**, contribute what you learned back. Every non-obvious insight — a pattern that worked, a dead end, a solc behavior that surprised you, a testing trick — should become a wiki entry tagged with `pattern_type` and `domain` so future agents can retrieve it. When ≥3 entries share the same `(pattern_type, domain)` pair, the platform auto-proposes promoting the pattern to a rule.

The orchestrator's **frontier** (the ranked queue of open tasks) is also wiki-backed. When you complete a task, the follow-up questions and next steps you identify become new frontier entries. When the frontier is empty, the keep-alive stream draws from research-sota — but a well-stocked frontier from your own follow-ups is always better than synthetic filler.

## Code quality bar

Every PR should be something a Paradigm reviewer would merge without hesitation. The bar is high because trust is earned per-PR, not per-session.

**Correctness.** The change does what it claims. The oracle tier is appropriate for the scope (a parser fix needs tier 3; a MIR pass needs tier 7+). Test fixtures exercise the exact behavior changed, not just "something nearby passes."

**Scope.** One logical change per PR. The blast radius fits the archetype: ≤5 files for a parser-slice, ≤3 for a diagnostic-port, ≤10 for a lowering-slice, ≤20 for a codegen-slice. If a change touches more than 2 crates, pause for human approval.

**Style.** AGENTS.md and CLAUDE.md are law:
- Conventional Commits: `feat(sema): …`, `fix(parse): …`
- Diagnostic style: no trailing periods, backtick code refs, `note`/`help`/`span_note` subdiagnostics
- Use `sym::name` or `kw::Keyword` instead of `.as_str()` on `Symbol`/`Ident`
- Visitor pattern: always call `walk_*` for child traversal; `type BreakValue = Never` if visitor never breaks
- Arena allocation for AST: no `Box<T>` outside arenas
- UI test annotations: `//~ ERROR:`/`WARN:`/`NOTE:`/`HELP:`
- No filler words: "comprehensive", "robust", "enhance", "streamline", "seamless" are banned

**Evidence.** The PR body carries the oracle evidence:
- Before/after test counts: "solc_syntax_tests: 2840/3551 → 2847/3551"
- Solc error code citation: "Ports TypeError 3726, ref `libsolidity/analysis/TypeChecker.cpp` L482 at sha `abc1234`"
- CodSpeed delta for perf-sensitive paths
- "CI passed" is not evidence. Named oracle results are.

**Reviewability.** A reviewer should understand the change, verify the evidence, and click merge in under 10 minutes. If the PR body would require follow-up questions, it's not ready. If the diff requires scrolling past 20 files, it's too big.

## Open questions

**Yul switch semantics** (paradigmxyz/solar#415): How should Yul `switch` fall-through map to HIR? Options: (a) preserve structure with explicit Default, (b) desugar to nested if. dipanshuhappy's draft #652 raises this. Humans weigh in before the yul-hir track ships beyond let/assign.
