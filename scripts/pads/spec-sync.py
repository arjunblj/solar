#!/usr/bin/env python3
"""
Sync/check the canonical .pads/spec.json against PADS.md.

Usage:
    python3 scripts/pads/spec-sync.py [--spec PATH] [--json PATH] [--write]

The JSON file is a machine-readable serialization of the PADS.md frontmatter
and free-form markdown body under `free_form_body`. This guard keeps the canonical
JSON import path aligned with the human-authored markdown brief.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    sys.stderr.write(
        "[spec-sync] PyYAML is required. Install with: python3 -m pip install pyyaml\n"
    )
    sys.exit(6)

FRONTMATTER_RE = re.compile(r"^---\n(.*?)\n---\n?(.*)$", re.DOTALL)


def parse_markdown_spec(source: str) -> dict:
    match = FRONTMATTER_RE.match(source)
    if not match:
        raise ValueError(
            "Document lacks a YAML frontmatter block bounded by '---' delimiters."
        )
    frontmatter = yaml.safe_load(match.group(1)) or {}
    if not isinstance(frontmatter, dict):
        raise ValueError("Frontmatter must parse to a mapping")
    body = match.group(2).lstrip("\n")
    if body.strip():
        frontmatter["free_form_body"] = body
    else:
        frontmatter.pop("free_form_body", None)
    return normalize_spec(frontmatter)


def parse_json_spec(source: str) -> dict:
    value = json.loads(source)
    if not isinstance(value, dict):
        raise ValueError(".pads/spec.json must contain a JSON object")
    return normalize_spec(value)


def normalize_spec(value: dict) -> dict:
    normalized = dict(value)
    body = normalized.get("free_form_body")
    if isinstance(body, str):
        stripped = body.strip("\n")
        if stripped.strip():
          normalized["free_form_body"] = stripped
        else:
          normalized.pop("free_form_body", None)
    elif body is None:
        normalized.pop("free_form_body", None)
    return normalized


def canonical_json(value: dict) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def pretty_json(value: dict) -> str:
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="PADS markdown/json sync guard")
    parser.add_argument("--spec", default="PADS.md")
    parser.add_argument("--json", dest="json_path", default=".pads/spec.json")
    parser.add_argument(
        "--write",
        action="store_true",
        help="Write the canonical JSON representation from PADS.md.",
    )
    args = parser.parse_args()

    spec_path = Path(args.spec)
    json_path = Path(args.json_path)
    if not spec_path.exists():
        sys.stderr.write(f"[spec-sync] Could not read {spec_path}\n")
        return 2

    try:
        expected = parse_markdown_spec(spec_path.read_text(encoding="utf-8"))
    except Exception as err:
        sys.stderr.write(f"[spec-sync] Failed to parse {spec_path}: {err}\n")
        return 3

    if args.write:
        if os.getenv("GITHUB_ACTIONS"):
            sys.stderr.write(
                "[spec-sync] Refusing to --write in CI. Run locally:\n"
                "  python3 scripts/pads/spec-sync.py --write\n"
            )
            return 7
        json_path.parent.mkdir(parents=True, exist_ok=True)
        json_path.write_text(pretty_json(expected), encoding="utf-8")
        print(f"[spec-sync] Wrote canonical JSON to {json_path}")
        return 0

    if not json_path.exists():
        sys.stderr.write(
            f"[spec-sync] Expected JSON file {json_path} is missing. "
            "Seed it with `python3 scripts/pads/spec-sync.py --write`.\n"
        )
        return 4

    try:
        actual = parse_json_spec(json_path.read_text(encoding="utf-8"))
    except Exception as err:
        sys.stderr.write(f"[spec-sync] Failed to parse {json_path}: {err}\n")
        return 5

    if canonical_json(actual) != canonical_json(expected):
        sys.stderr.write(
            "\n".join(
                [
                    "[spec-sync] PADS markdown/json drift detected.",
                    f"  spec: {spec_path}",
                    f"  json: {json_path}",
                    "",
                    "Run the following locally and commit both files together:",
                    "  python3 scripts/pads/spec-sync.py --write",
                    "  python3 scripts/pads/tier0-guard.py --write",
                    "",
                ]
            )
        )
        return 1

    print(f"[spec-sync] OK {json_path}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as err:  # noqa: BLE001
        sys.stderr.write(f"[spec-sync] Unexpected failure: {err}\n")
        sys.exit(8)
