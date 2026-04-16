#!/usr/bin/env python3
"""
Append-only guard for PADS.md ecosystems.

For each file listed below, verifies that the NEW version is an append-only
extension of the OLD version (prior lines preserved; only new lines may be
added at the end). Scopes:

  - pads/lessons.jsonl         — whole file is append-only.
  - pads/rejected-ideas.md     — whole file is append-only.
  - PADS.md                    — only the `## Change History` section is
                                 append-only. Outside that section, edits
                                 are fine.

Runs against `git diff --cached` by default (pre-commit). CI can run against
`git diff origin/main...HEAD` by passing --range.

Depends on: git in PATH. Only stdlib.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys

DEFAULT_PATHS = [
    "pads/lessons.jsonl",
    "pads/rejected-ideas.md",
    "PADS.md",
]

CHANGE_HISTORY_HEADING = re.compile(r"^#+\s*Change History\b", re.IGNORECASE)
ANY_HEADING = re.compile(r"^#{1,6}\s")


def git_show(ref: str, path: str) -> str:
    """Return file contents at ref; empty string if the file did not exist."""
    try:
        return subprocess.check_output(
            ["git", "show", f"{ref}:{path}"],
            text=True,
            stderr=subprocess.DEVNULL,
        )
    except subprocess.CalledProcessError:
        return ""


def current_file(path: str, range_spec: str | None) -> str:
    """Return the NEW version of path: either worktree or tip of HEAD."""
    if range_spec:
        return git_show("HEAD", path)
    try:
        # Staged copy, if any, else working tree.
        return subprocess.check_output(
            ["git", "show", f":{path}"],
            text=True,
            stderr=subprocess.DEVNULL,
        )
    except subprocess.CalledProcessError:
        try:
            with open(path, encoding="utf-8") as f:
                return f.read()
        except OSError:
            return ""


def base_ref(range_spec: str | None) -> str:
    if range_spec:
        left = range_spec.split("...")[0].split("..")[0]
        return left or "HEAD"
    return "HEAD"


def extract_change_history(content: str) -> str:
    """Return just the Change History section body (excluding the heading itself)."""
    lines = content.splitlines()
    out: list[str] = []
    in_section = False
    for line in lines:
        if CHANGE_HISTORY_HEADING.match(line):
            in_section = True
            continue
        if in_section and ANY_HEADING.match(line):
            break
        if in_section:
            out.append(line)
    # Strip trailing blank lines so single whitespace drift is ignored.
    while out and not out[-1].strip():
        out.pop()
    return "\n".join(out)


def check_append_only(old: str, new: str, scope_describe: str) -> str | None:
    """Return an error string if `new` is not an append-only extension of `old`."""
    if not old.strip():
        # No prior content to preserve.
        return None
    old_trimmed = old.rstrip()
    new_trimmed = new.rstrip()
    if not new_trimmed.startswith(old_trimmed):
        return (
            f"{scope_describe}: NEW content is not an append-only extension of OLD.\n"
            f"  OLD prefix expected but not found.\n"
            "  If you need to correct an earlier entry, APPEND a new record that supersedes it."
        )
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description="PADS append-only guard")
    parser.add_argument("--range", dest="range_spec", default=None)
    parser.add_argument("--path", action="append", default=[])
    args = parser.parse_args()

    paths = args.path if args.path else DEFAULT_PATHS
    base = base_ref(args.range_spec)

    violations: list[str] = []
    for path in paths:
        old = git_show(base, path)
        new = current_file(path, args.range_spec)

        if path.endswith("PADS.md"):
            error = check_append_only(
                extract_change_history(old),
                extract_change_history(new),
                f"{path} ## Change History",
            )
        else:
            error = check_append_only(old, new, path)

        if error:
            violations.append(error)

    if violations:
        sys.stderr.write(
            "\n".join(
                [
                    "[append-only-guard] Append-only invariant broken:",
                    *[f"  * {v}" for v in violations],
                    "",
                ]
            )
        )
        return 1

    print("[append-only-guard] OK")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as err:  # noqa: BLE001
        sys.stderr.write(f"[append-only-guard] Unexpected failure: {err}\n")
        sys.exit(2)
