# Solar PADS Directory

This directory is the machine-readable and operator-facing companion to
`PADS.md`. `PADS.md` is the source of truth; files here make the same
context easier for an autonomous harness to ingest without guessing.

## Files

- `spec.json` is generated from `PADS.md` frontmatter plus body by
  `scripts/pads/spec-sync.py`.
- `tier0.sha256` is the checksum for the Tier-0 constitution fields in
  `PADS.md`: `project`, `hard_constraints`, `scope_of_autonomy`, and
  `edit_rules`.
- `setup.sh` bootstraps the Rust/PADS toolchain, validates `PADS.md`,
  reports tool versions, and fetches locked Cargo dependencies.
- `rules/oracles.md` records runnable commands, proof tiers, snapshot
  hygiene, and performance no-claim rules.
- `rules/pr-rubric.md` defines what a publishable autonomous PR must
  contain and when it should be rejected or turned into a blocker.
- `rules/upstream-map.md` summarizes upstream intent, branch watchlists,
  and reference-only surfaces.
- `rules/feature-matrix.md` captures high-risk Solidity language and EVM
  feature families that require differential evidence.
- `rules/foundry-readiness.md` describes the Foundry/Hardhat process,
  artifact, source-map, cache, and backend-readiness contract.
- `rules/performance.md` scopes performance work to correctness-gated
  measurements and bounded experiments.

## Edit Flow

When changing project policy or durable campaign context:

1. Edit `PADS.md` first.
2. Run `python3 scripts/pads/spec-sync.py --write`.
3. If Tier-0 fields changed, run `python3 scripts/pads/tier0-guard.py --write`.
4. Run `python3 scripts/pads/validate.py`.
5. Commit `PADS.md`, `.pads/spec.json`, and `.pads/tier0.sha256`
   together when applicable.

When changing only rule files under `.pads/rules`, keep the change
consistent with `PADS.md`; if the rule introduces a new durable policy,
promote the summary into `PADS.md` in the same PR.

## Kickoff Contract

A fresh harness should read `PADS.md`, `AGENTS.md`, this manifest, and
the rule files before creating issues or dispatching workers. The goal
is not to copy these documents into a backlog. The goal is to generate a
repo-grounded branch train whose PRs each prove a reviewable compiler
invariant with the strongest honest oracle available.
