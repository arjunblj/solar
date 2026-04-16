# pads/rules/

Promoted rules. A rule lives here when ≥ 3 lessons in `pads/lessons.jsonl`
share the same `(pattern_type, domain)` tuple and a PR has promoted the
pattern to a rule (see `PADS.md` revision protocol).

Rule files have YAML frontmatter with a `paths:` glob so Claude Code's
path-scoped rules mechanism can lazy-load them. Example:

```markdown
---
paths:
  - crates/sema/src/typeck/**
---

# Rule: prefer solc error code lookups over hand-rolled messages

When diagnosing a type mismatch, first check solc's error code for the
exact case. Cite the solc source file + SHA in the PR body.
```

This directory is empty at initial seeding. Rules land as they are earned.
