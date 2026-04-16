---
name: conventional-commit
description: Format commit messages to Conventional Commits + the Solar `(scope)` convention. Required for every commit in this fork.
tier: style
---

# conventional-commit

## When

- Every commit, without exception.

## Canonical format

```
<type>(<crate>): <short summary>

<optional body>

<optional footer, e.g. Co-authored-by: ...>
```

`<type>` is one of: `feat`, `fix`, `docs`, `chore`, `refactor`, `perf`,
`test`, `build`, `ci`, `style`, `revert`.

`<crate>` is the crate touched (e.g. `sema`, `parse`, `hir`, `codegen`,
`tester`). When multiple crates are touched, use the most-impacted one or
`workspace`.

Subject ≤ 72 chars. No trailing period.

## Examples

```
feat(sema): add view/pure check for pure-function override

Closes part of #615. Ports solc TypeError 3726 with a matching fixture in
tests/ui/typeck/view-pure/override.sol.

Co-authored-by: padsdotdev[bot] <3385788+padsdotdev[bot]@users.noreply.github.com>
```

```
fix(parse): reject trailing comma in tuple type
```

## Test case

Validate a commit with commitlint locally:

```bash
echo "feat(sema): example" | npx --no-install @commitlint/cli --extends '@commitlint/config-conventional'
```

## Notes

- The pr-rubric linter in `src/lib/pr-rubric.ts` (pads-hack side) flags
  non-conventional titles as `title_not_conventional`.
- Do not prefix with `pads:`; the `(pads)` scope is reserved for
  infrastructure-only changes.
