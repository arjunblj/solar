# Rejected Ideas

Append-only. Each entry cites the decision + a `Revisit-if:` clause. Agents
must not re-derive these. If the `Revisit-if` condition fires, open a PR
proposing reconsideration; do not silently act.

---

## Do not migrate Solar to a salsa-style query engine

**Decision date:** 2026-04-16
**Cited by:** upstream `paradigmxyz/solar#91`, @DaniPopes skepticism; matklad
Feb 2026 "Against Query Based Compilers".
**Why rejected:**
- matklad's piece + Solar maintainer skepticism on `#91` argue against wholesale
  salsa adoption.
- Solar's current one-shot `cached!` memoizer is adequate for the current
  pipeline; a salsa retrofit is a multi-week cross-cutting change with
  unclear win.
- Solar is a batch compiler; incremental query re-execution is more valuable
  for IDE workloads (and is at least partially planned for via the LSP
  track).
**Revisit-if:**
- LSP track hits a wall that can only be unlocked by incremental dependency
  tracking, AND the maintainers on `paradigmxyz/solar` reverse their position.
- Cross-crate compilation introduces a problem that the current memoizer
  cannot express (e.g. multi-crate import resolution with inter-crate
  invalidation).
- Research-sota track experiments with salsa under `research/` produce a
  proof-of-concept that demonstrates a clear perf or developer-ergonomics
  win the current design cannot match.

## Do not adopt Venom directly as Solar's IR

**Decision date:** 2026-04-16
**Cited by:** plan §2 (#687 Phase 2 analysis); gakonst MIR roadmap already
specifies the Solar MIR shape.
**Why rejected:**
- Venom is Vyper's IR, designed for Vyper's frontend. Solar's MIR should be
  shaped by Solidity's HIR, not retrofit onto a Vyper-shaped tree.
- Direct adoption would make Solar's MIR layer fragile to Vyper changes.
- Pattern literature (Cranelift, Sonatina) gives Solar the right primitives
  to design its own MIR. Borrow ideas, not trees.
**Revisit-if:**
- A future MIR pass discovers an abstraction in Venom that demonstrably
  reduces Solar's pass count by ≥ 30% and generalizes correctly.
- Vyper and Solar agree on a shared stable IR (explicitly signaled by both
  projects' maintainers, not just pads-side enthusiasm).

## Do not full-equality-saturate the Yul optimizer

**Decision date:** 2026-04-16
**Cited by:** Chris Fallin April 2026 Cranelift retrospective
(<https://cfallin.org/blog/2026/04/09/aegraph/>).
**Why rejected:**
- Fallin's piece is explicit: for a ruleset in the hundreds, full
  equality-saturation buys ~0.1% over eager rewrites.
- The acyclic e-graph + sea-of-nodes-with-CFG + ISLE + eager rewrites +
  cost-based extraction pattern gets ~2% over classical pipelines — that's
  the target for Stream J.
- egglog-style equality saturation doesn't carry its weight at Solar's
  ruleset size.
**Revisit-if:**
- Stream L's superoptimizer surfaces cases where equality saturation
  genuinely finds rewrites the aegraph approach misses.
- A future ruleset explosion (>1000 rules) changes the cost calculus.

## Do not bootstrap self-hosting Solar before codegen is usable

**Decision date:** 2026-04-16
**Cited by:** plan §13 defer list; rustc bootstrap analogy.
**Why rejected:**
- Self-hosting is a multi-year workstream. Solar can't compile itself until
  codegen emits correct EVM bytecode and the runtime oracle is green.
- Self-hosting before codegen means writing Solar in a language it cannot
  yet compile — it adds translation burden without unblocking anything.
- Bootstrap is a 2027+ aspiration, not a 2026 target.
**Revisit-if:**
- Stream D (MIR passes) + Stream F (bytecode equivalence) both stabilize,
  AND codegen covers ≥80% of `corpus.execution`, AND the maintainers agree
  the payoff beats the translation cost.

## Do not rebless `testdata/solidity/**` snapshots to smooth out CI

**Decision date:** 2026-04-16
**Cited by:** `tier0.hard_constraints` + AGENTS.md.
**Why rejected:**
- `testdata/solidity/` is the pinned community submodule. Blessing its
  snapshots in-fork is a reward-hack: it appears to improve compatibility
  while actually breaking the oracle.
- Upstream solc is the source of truth for those fixtures. Any disagreement
  is either a real divergence to document (via `solar-divergence.md`) or a
  Solar bug to fix.
**Revisit-if:**
- Upstream solc ships an actual breaking change that requires fixture
  updates. Those come via a submodule SHA bump, not via a rebless.

## Do not edit `.stderr` snapshots without a matching source change

**Decision date:** 2026-04-16
**Cited by:** `tier0.hard_constraints`, EvilGenie (arXiv:2511.21654).
**Why rejected:**
- The canonical reward hack in SWE-agent evaluations is "edit the expected
  output to match my output". The reward-hack pre-flight linter blocks this
  at integration time; humans must not relax the rule to make CI easier.
- A genuine behavior change will always come with a source-side diff that
  explains *why* the snapshot changed. A snapshot-only diff is almost
  always an attempt to hide a regression.
**Revisit-if:**
- A snapshot normalization pass is proposed that is purely cosmetic (e.g.
  path format changes) AND is opt-in AND has a matching reblessed fixture
  script AND gets human sign-off.
