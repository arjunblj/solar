#!/usr/bin/env python3
"""
Tier-0 guard for PADS.md v2 specs.

Hashes the Tier-0 blob (project + hard_constraints + scope_of_autonomy +
edit_rules) from the target PADS.md and compares it to the expected hash
stored at .pads/tier0.sha256. Exits non-zero if they differ.

Canonicalization: JSON Canonicalization Scheme (RFC 8785 subset) — sorted
keys, no whitespace. Matches the TypeScript implementation in pads-hack's
src/lib/import/pads-spec-io.ts so hashes are bit-identical across languages.

Usage:
    python3 scripts/pads/tier0-guard.py [--spec PATH] [--expected PATH] [--write]

Depends on: PyYAML (pip install pyyaml).
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    sys.stderr.write(
        "[tier0-guard] PyYAML is required. Install with: python3 -m pip install pyyaml\n"
    )
    sys.exit(6)

TIER0_FIELDS = ("project", "hard_constraints", "scope_of_autonomy", "edit_rules")
FRONTMATTER_RE = re.compile(r"^---\n(.*?)\n---\n?(.*)$", re.DOTALL)


def parse_frontmatter(source: str) -> dict:
    match = FRONTMATTER_RE.match(source)
    if not match:
        raise ValueError(
            "Document lacks a YAML frontmatter block bounded by '---' delimiters."
        )
    return yaml.safe_load(match.group(1)) or {}


def canonical_json(value) -> str:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    )


def hash_tier0(spec: dict) -> str:
    tier0 = spec.get("tier0")
    if not isinstance(tier0, dict):
        raise ValueError("tier0 block missing or not a mapping")
    picked = {k: tier0[k] for k in TIER0_FIELDS if k in tier0}
    canonical = canonical_json(picked)
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description="PADS Tier-0 guard")
    parser.add_argument("--spec", default="PADS.md")
    parser.add_argument("--expected", default=".pads/tier0.sha256")
    parser.add_argument(
        "--write",
        action="store_true",
        help="Write the current hash to --expected instead of checking.",
    )
    args = parser.parse_args()

    spec_path = Path(args.spec)
    if not spec_path.exists():
        sys.stderr.write(f"[tier0-guard] Could not read {spec_path}\n")
        return 2

    try:
        spec = parse_frontmatter(spec_path.read_text(encoding="utf-8"))
    except Exception as err:
        sys.stderr.write(f"[tier0-guard] Failed to parse PADS.md: {err}\n")
        return 3

    try:
        actual = hash_tier0(spec)
    except ValueError as err:
        sys.stderr.write(f"[tier0-guard] {err}\n")
        return 3

    expected_path = Path(args.expected)

    if args.write:
        expected_path.parent.mkdir(parents=True, exist_ok=True)
        expected_path.write_text(f"{actual}\n", encoding="utf-8")
        sys.stderr.write(f"[tier0-guard] Wrote hash {actual} to {expected_path}\n")
        return 0

    if not expected_path.exists():
        sys.stderr.write(
            f"[tier0-guard] Expected hash file {expected_path} is missing. "
            "Seed it with `python3 scripts/pads/tier0-guard.py --write` from a CODEOWNER commit.\n"
        )
        return 4

    expected = expected_path.read_text(encoding="utf-8").strip()

    if expected != actual:
        sys.stderr.write(
            "\n".join(
                [
                    "[tier0-guard] Tier-0 constitution drift detected.",
                    f"  expected: {expected}",
                    f"  actual:   {actual}",
                    "",
                    "Tier-0 sections (project, hard_constraints, scope_of_autonomy, edit_rules)",
                    "are human-only. If this change is intentional, a CODEOWNER must run:",
                    "  python3 scripts/pads/tier0-guard.py --write",
                    "and commit the updated hash with a human-approved label.",
                    "",
                ]
            )
        )
        return 1

    print(f"[tier0-guard] OK {actual}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as err:  # noqa: BLE001
        sys.stderr.write(f"[tier0-guard] Unexpected failure: {err}\n")
        sys.exit(5)
