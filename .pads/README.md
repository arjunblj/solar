# .pads/

Project-specific harness configuration for Solar's autonomous campaign on `arjunblj/solar`.

`PADS.md` (in the repository root) is the canonical project spec. Files in this directory are supporting artifacts:

- `setup.sh` — sandbox bootstrap script. Installs rust 1.95.0 + nightly, cargo-nextest, typos, forge, solc 0.8.31, and warms the cargo cache. Idempotent; safe to run inside a pre-warmed E2B template.
- `rules/pr-rubric.md` — the PR-shape rubric workers and reviewers use to judge whether a PR is mergeable. Drop-in compatible with the harness's `pr_rubric` configuration.
- `rules/pr-shape.md` — winning PR body templates with concrete examples (winning vs losing).
- `rules/always.md` — minimal always-on rules for path-scoped guardrails.

The canonical reader is the harness in `tempo-hack`; it parses `PADS.md`'s YAML frontmatter directly via `parsePadsSpec` / `PadsMdSchemaV2`. There is no separate `spec.json` mirror to maintain.

To verify the spec parses:

```bash
bash .pads/setup.sh --check
```
