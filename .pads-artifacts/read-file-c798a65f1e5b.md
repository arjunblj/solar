    1|# solar
    2|
    3|> Build Solar into a serious alternative to solc — a Rust Solidity compiler with correct frontend semantics, meaningful type-checking parity, high-quality diagnostics, competitive performance, and eventually full MIR + EVM codegen with runtime-verifiable output. Push beyond the published roadmap into 2024–2026 compiler SOTA (aegraphs, translation validation, superoptimization, metamorphic fuzz, LLM rewrite-rule synthesis) where it makes sense.
    4|
    5|
    6|## Objective
    7|- Metric: Not yet synthesized (satisfy)
    8|- Quality gate: hybrid
    9|- Current best: No best result captured yet
   10|
   11|
   12|## Setup
   13|```sh
   14|git clone https://github.com/arjunblj/solar.git && cd solar
   15|git clone https://github.com/arjunblj/solar.git && cd solar
   16|
   17|
   18|# test
   19|cargo test
   20|
   21|# lint
   22|cargo clippy --workspace --all-targets
   23|```
   24|
   25|## Repository
   26|- URL: https://github.com/arjunblj/solar
   27|- Clone: `git clone https://github.com/arjunblj/solar.git && cd solar`
   28|
   29|## Workstreams
   30|- Type checker (high)
   31|  scope: crates/sema/src/typeck/**
   32|- Yul → HIR lowering (high)
   33|  scope: crates/parse/src/yul/**, crates/sema/src/hir/**
   34|- Testing infra + oracles (high)
   35|  scope: tools/tester/**, tests/**, fuzz/**
   36|- Diagnostic quality + parity (medium)
   37|  scope: crates/interface/**, crates/sema/src/**
   38|- MIR + EVM codegen (high)
   39|  scope: crates/codegen/**, crates/mir/**
   40|- Bytecode equivalence harness (high)
   41|  scope: tools/diff-harness/**
   42|- Performance (medium)
   43|  scope: crates/**, benches/**
   44|- Fuzzing + hardening (low)
   45|  scope: fuzz/**
   46|- LSP (low)
   47|  scope: crates/lsp/**
   48|- Novel SOTA research (low)
   49|  scope: research/**
   50|
   51|## Verification
   52|- Quality gate: hybrid
   53|- Test command: cargo test
   54|- Lint command: cargo clippy --workspace --all-targets
   55|- Additional commands: cargo check --workspace · cargo build --workspace · cargo nextest run --workspace · cargo uitest · cargo clippy --workspace --all-targets -- -D warnings · cargo +nightly fmt --all --check · typos --format brief · TESTER_MODE=solc-solidity cargo nextest run -p solar-compiler --test tests · TESTER_MODE=solc-yul cargo nextest run -p solar-compiler --test tests · cargo test -p solar-tester · cargo codspeed build && cargo codspeed run
   56|
   57|
   58|## Context
   59|- Architecture: Owned baseline: arjunblj/solar@main (10283ae5c165) | Entrypoints: .github/scripts/install_iai_callgrind_runner.sh, benches/analyze/main.py | Config files: Cargo.toml, benches/Cargo.toml, examples/Cargo.toml, benches/analyze/pyproject.toml, crates/ast/Cargo.toml, crates/cli/Cargo.toml, crates/config/Cargo.toml, crates/data-structures/Cargo.toml | Important paths: .pads/spec.json, PADS.md, AGENTS.md, CLAUDE.md, README.md, Cargo.toml, benches/README.md, .github/workflows/ci.yml
   60|- Key issues: Owned implementation baseline is arjunblj/solar@main (10283ae5c165)., Use paradigmxyz/solar docs, issues, merged history, and open PRs as reference material only. Do not assume upstream unmerged code exists in this fork.
   61|- Reference docs: .pads/spec.json, PADS.md, AGENTS.md, CLAUDE.md, README.md, Cargo.toml, benches/README.md, .github/workflows/ci.yml
   62|- Notes: Loaded 8 repository context files for grounding. Upstream references from paradigmxyz/solar are guidance only and must not be treated as code already present in this fork.
   63|
   64|# Solar Completion Program
   65|
   66|This document is the operating constitution for a long autonomous run whose goal is to complete Solar as a production-grade Solidity compiler. It is not a list of tiny starter edits. It tells the orchestrator how to reason, what to prove, where to look upstream, how to phase the work, and which evidence makes a PR worth opening.
   67|
   68|The run should start with no user mission. The mission is this file.
   69|
   70|## North Star
   71|
   72|Solar becomes serious when it can compile real Solidity projects with solc-compatible behavior, expose a maintainable Rust compiler architecture, and produce upstreamable PRs whose evidence is strong enough that a Paradigm reviewer can merge them without asking basic follow-up questions.
   73|
   74|Correctness comes first. Performance is only meaningful once the compiler is proving the right behavior against solc, production contracts, and eventually runtime execution.
   75|
   76|## Orchestrator Handoff
   77|
   78|The orchestrator should treat this file as the mission and planning brief. Do not rely on an external run prompt. Do not optimize for "opened a PR." Optimize for a multi-month sequence of upstreamable changes that monotonically increases measured correctness.
   79|
   80|The first no-mission run should:
   81|
   82|1. Bootstrap the repo with `.pads/setup.sh`.
   83|2. Read this `PADS.md`, `AGENTS.md`, upstream `paradigmxyz/solar` issues/PRs/branches, and local fork diffs.
   84|3. Seed the frontier from `starter_tasks`, but refine them against current repo evidence before dispatching implementers.
   85|4. Start in Phase 0 and Phase 1. Do not dispatch codegen, bytecode-equivalence, or performance implementation work until the relevant correctness harnesses exist.
   86|5. Prefer fewer, larger, real PRs over tiny motion PRs. A meaty PR can be hundreds or thousands of lines if it is dependency-sliced, tested, and reviewable.
   87|6. Keep branch-import work explicit: reference branches first, integration branches second, cherry-picks only when small and tested.
   88|7. Preserve context. Every completed or blocked task should leave behind a frontier entry that names the next dependency, oracle, and files.
   89|
   90|The planner has freedom to choose the exact next slice. The boundaries are: correctness before performance; reference upstream but write only to this fork; every behavioral claim needs an oracle; and every PR should move Solar measurably closer to production Solidity compatibility.
   91|
   92|## Current State
   93|
   94|The fork is essentially upstream `main` plus Pads guardrails. Treat local fork state as authoritative. Treat `paradigmxyz/solar` issues, PRs, and branches as reference material unless explicitly imported into this fork.
   95|
   96|Mainline Solar today is mostly frontend:
   97|
   98|```text
   99|lexer -> parser -> AST -> HIR -> sema/typeck -> diagnostics/ABI output
  100|```
  101|
  102|What exists:
  103|
  104|- `solar-parse`: lexer, parser, Yul parser, parser diagnostics.
  105|- `solar-ast`: AST definitions, visitors, JSON-ish emission surfaces.
  106|- `solar-sema`: resolver, semantic analysis, gated type checker.
  107|- `solar-interface`: source map, diagnostics, sessions.
  108|- `solar-cli`, `solar-config`, `solar-macros`, `solar-data-structures`.
  109|- `tools/tester`: UI and solc corpus runner.
  110|- `benches`: Criterion, CodSpeed, and iai-callgrind benchmark surfaces.
  111|
  112|What is not yet mainline-complete:
  113|
  114|- Broad `-Ztypeck` solc corpus exposure and semantic parity.
  115|- NatSpec and frontend emission parity.
  116|- Inline assembly / Yul lowering into HIR.
  117|- MIR, HIR-to-MIR lowering, EVM codegen, stack scheduling, assembler.
  118|- Bytecode equivalence harness.
  119|- Runtime equivalence through Foundry/Anvil/revm.
  120|- LSP built on stable typeck.
  121|- Production-corpus correctness and performance gates.
  122|
  123|## Phase Model
  124|
  125|The orchestrator should keep this ordering unless a blocker forces preparatory work.
  126|
  127|### Phase 0: Setup, Import Intelligence, And Measurement
  128|
  129|Goal: make every future claim measurable.
  130|
  131|Required outcomes:
  132|
  133|- Sandbox setup reliably initializes Rust, `cargo-nextest`, `typos`, PADS guards, and the pinned `testdata/solidity` submodule.
  134|- The run imports upstream issue/branch intelligence into the Pads frontier.
  135|- Corpus commands point at the correct package and test target.
  136|- Production corpora are cloned, cached, and smoke-compiled in a way the orchestrator can shard later.
  137|- The first PRs may be harness/setup PRs, but they must unlock stronger evidence.
  138|
  139|Do not skip this phase. If the harness cannot measure correctness, implementation PRs will become motion.
  140|
  141|### Phase 1: Frontend Correctness
  142|
  143|Goal: make Solar credible on parse, diagnostics, AST, ABI, NatSpec, and semantic/type checking before claiming full compiler correctness.
  144|
  145|Workstreams:
  146|
  147|- Parser and diagnostics parity against owned UI fixtures and solc parser corpus.
  148|- Type checker parity under `-Ztypeck`, centered on `paradigmxyz/solar#615`, `#663`, and PR `#737` as reference.
  149|- Diagnostic quality ports from solc error codes into Solar `error_code!(NNNN)` plus fixtures.
  150|- AST / ABI / NatSpec Standard JSON parity for selected contracts.
  151|- Skip-list reduction in `tools/tester/src/solc/solidity.rs` and `tools/tester/src/solc/yul.rs`.
  152|
  153|Representative meaty PRs:
  154|
  155|- Enable one category of solc TypeError tests under `-Ztypeck`, keep counts, and port owned UI fixtures for the first gaps.
  156|- Implement argument-aware overload resolution with fixtures for ambiguous calls, named args, and member overloads.
  157|- Implement call-kind parity for array `push`, call options, and `require(cond, CustomError(...))`.
  158|- Add an AST/NatSpec parity harness for a small stable corpus and fix the first local emission mismatch.
  159|
  160|### Phase 2: Yul, HIR, MIR, Codegen, And Runtime Correctness
  161|
  162|Goal: get from frontend correctness to executable EVM output with a proof path.
  163|
  164|Reference branch:
  165|
  166|- `paradigmxyz/solar:feat/codegen-mir` / PR `#693` is the major codegen branch. It is not a blind merge target. It is a reference branch to extract architecture and reviewable slices from.
  167|- PR `#749` was merged into `feat/codegen-mir`, not `main`. Account for that branch reality.
  168|
  169|Dependency order:
  170|
  171|1. Yul/inline assembly to HIR boundary (`#415`). Prefer the branch's dedicated `hir::yul` direction as reference, but land reviewable slices.
  172|2. MIR-only infrastructure, text format/parser, validator, and pass manager.
  173|3. HIR-to-MIR lowering for arithmetic, locals, branches, loops, returns, and minimal builtins.
  174|4. Liveness (`#694`) and phi elimination (`#695`).
  175|5. Stack model <=16 (`#696`), then full stack scheduling/spilling (`#697`).
  176|6. Assembler label/jump resolution (`#698`).
  177|7. First executable bytecode.
  178|8. Bytecode equivalence harness (`#704`) with Foundry/Anvil/revm.
  179|9. Complex types, storage, calldata, ABI, events, constructors, calls (`#699` and branch commits).
  180|10. Optimizer passes only after runtime oracles exist: DCE (`#700`), constant folding (`#701`), SCCP (`#702`), CSE (`#703`).
  181|
  182|Do not claim codegen progress unless the PR states exactly which runtime/equivalence tier is available and which is still missing.
  183|
  184|### Phase 3: Production Solidity Correctness
  185|
  186|Goal: compile real Solidity projects, not just fixtures.
  187|
  188|Corpus phases:
  189|
  190|- Phase 3A: `forge-std`, small Foundry fixture projects, direct single-file remapping smoke.
  191|- Phase 3B: OpenZeppelin, Solmate, PRBMath.
  192|- Phase 3C: Solady, Uniswap v3, Uniswap v4.
  193|- Phase 3D: Aave v3, Maker DSS, Compound v2/v3.
  194|- Phase 3E: EigenLayer and other large modern Foundry projects once sharding/caching are stable.
  195|
  196|Use framework builds to test project layout, remappings, build-info, profiles, and import resolution. Use direct Standard JSON inputs to compare solc and Solar outputs. Do not use production corpora as a first PR gate for small parser fixes, but always move serious correctness claims toward these corpora.
  197|
  198|### Phase 4: Performance
  199|
  200|Goal: preserve and improve Solar's speed after correctness has a floor.