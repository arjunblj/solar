---
name: gh-pr-draft
description: Open a draft PR with a rubric-compliant body. Default for agent-authored PRs; flip to Ready only when all oracles pass and the rubric linter is clean.
tier: workflow
---

# gh-pr-draft

## When

- Every agent-authored PR, at least until the rubric-driven linter is clean.
- Any PR where the CodSpeed delta is worse than the `-5%` budget (automatic).

## Canonical invocation

```bash
gh pr create \
  --draft \
  --base main \
  --title "$TRACK($CRATE): one-line description <= 72 chars" \
  --body-file /tmp/pr-body.md
```

Flip to ready once all oracles pass:

```bash
gh pr ready "$PR_NUMBER"
```

## PR body template

See `PADS.md` section `pr_rubric.sections` for the required blocks:
`summary`, `rationale`, `oracle_evidence`, `risk`, `follow_ups`. Plus the
`must_include` blocks: `Solc Compatibility Review`, `Before/After`,
`CodSpeed delta`.

## Test case

In a feature branch with 1 commit:

```bash
gh pr create --draft --base main --fill --dry-run
```

Should preview the PR without opening it.

## Notes

- Conventional Commits subject lint applies: `feat(scope): …`, `fix(…): …`.
- AI disclosure is required: see `pr_rubric.disclose_ai_text`.
- Branch naming: `pads/<track>/<slug>` per `pr_rubric.branch_naming`.
