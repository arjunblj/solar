#!/usr/bin/env python3
"""
Structural validator for the Solar PADS spec.

This is intentionally narrower than a full JSON Schema. It checks the
relationships the autonomous harness relies on during kickoff: tracks point
at real oracles, oracles point at real corpora, the priority order covers real
tracks, and reference rule files exist.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


ALLOWED_ORACLE_TIERS = {"prerequisite", "gate", "advisory"}
ALLOWED_WATCHLIST_MODES = {
    "track",
    "port",
    "extract",
    "cherry_pick",
    "ignore_until_cited",
}
REQUIRED_RULE_FILES = (
    ".pads/README.md",
    ".pads/rules/oracles.md",
    ".pads/rules/pr-rubric.md",
    ".pads/rules/upstream-map.md",
    ".pads/rules/feature-matrix.md",
    ".pads/rules/foundry-readiness.md",
    ".pads/rules/performance.md",
)


def load_json(path: Path) -> dict:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        raise ValueError(f"missing JSON file: {path}") from None
    except json.JSONDecodeError as err:
        raise ValueError(f"invalid JSON in {path}: {err}") from None
    if not isinstance(value, dict):
        raise ValueError(f"{path} must contain a JSON object")
    return value


def require_mapping(value: object, name: str) -> dict:
    if not isinstance(value, dict):
        raise ValueError(f"{name} must be a mapping")
    return value


def require_list(value: object, name: str) -> list:
    if not isinstance(value, list) or not value:
        raise ValueError(f"{name} must be a non-empty list")
    return value


def ids(items: list, name: str) -> set[str]:
    seen: set[str] = set()
    for index, item in enumerate(items):
        if not isinstance(item, dict):
            raise ValueError(f"{name}[{index}] must be a mapping")
        item_id = item.get("id")
        if not isinstance(item_id, str) or not item_id:
            raise ValueError(f"{name}[{index}].id must be a non-empty string")
        if item_id in seen:
            raise ValueError(f"duplicate {name} id: {item_id}")
        seen.add(item_id)
    return seen


def validate(spec: dict, repo_root: Path) -> list[str]:
    errors: list[str] = []

    tier0 = require_mapping(spec.get("tier0"), "tier0")
    project = require_mapping(tier0.get("project"), "tier0.project")
    for field in ("slug", "mission", "upstream", "baseline_commit"):
        if field not in project:
            errors.append(f"tier0.project.{field} is required")
    for field in ("hard_constraints", "edit_rules"):
        try:
            require_list(tier0.get(field), f"tier0.{field}")
        except ValueError as err:
            errors.append(str(err))
    scope = require_mapping(tier0.get("scope_of_autonomy"), "tier0.scope_of_autonomy")
    for field in ("permitted_subgoals", "must_pause_for_approval", "high_risk_prohibitions"):
        try:
            require_list(scope.get(field), f"tier0.scope_of_autonomy.{field}")
        except ValueError as err:
            errors.append(str(err))

    tracks = require_list(spec.get("tracks"), "tracks")
    oracles = require_list(spec.get("oracles"), "oracles")
    corpora = require_list(spec.get("corpora"), "corpora")
    track_ids = ids(tracks, "tracks")
    oracle_ids = ids(oracles, "oracles")
    corpus_ids = ids(corpora, "corpora")

    priority_order = require_list(spec.get("priority_order"), "priority_order")
    priority_ids = set(priority_order)
    if any(not isinstance(item, str) or not item for item in priority_order):
        errors.append("priority_order entries must be non-empty strings")
    unknown_priority = priority_ids - track_ids
    missing_priority = track_ids - priority_ids
    if unknown_priority:
        errors.append(f"priority_order references unknown tracks: {sorted(unknown_priority)}")
    if missing_priority:
        errors.append(f"priority_order omits tracks: {sorted(missing_priority)}")

    for track in tracks:
        for oracle_id in track.get("required_oracles", []):
            if oracle_id not in oracle_ids:
                errors.append(f"track {track['id']} references unknown oracle {oracle_id}")

    for oracle in oracles:
        tier = oracle.get("tier")
        if tier not in ALLOWED_ORACLE_TIERS:
            errors.append(f"oracle {oracle['id']} has invalid tier {tier!r}")
        corpus_ref = oracle.get("corpus_ref")
        if corpus_ref is not None and corpus_ref not in corpus_ids:
            errors.append(f"oracle {oracle['id']} references unknown corpus {corpus_ref}")
        if "command" not in oracle and "status" not in oracle:
            errors.append(f"oracle {oracle['id']} lacks both command and status")

    branch_policy = require_mapping(spec.get("branch_policy"), "branch_policy")
    watchlist = require_list(branch_policy.get("watchlist"), "branch_policy.watchlist")
    watchlist_ids = ids(watchlist, "branch_policy.watchlist")
    if not watchlist_ids:
        errors.append("branch_policy.watchlist must not be empty")
    for item in watchlist:
        mode = item.get("mode")
        if mode not in ALLOWED_WATCHLIST_MODES:
            errors.append(f"watchlist {item['id']} has invalid mode {mode!r}")

    sandbox = require_mapping(spec.get("sandbox_profile"), "sandbox_profile")
    for field in ("required_bins", "bootstrap_hooks"):
        try:
            require_list(sandbox.get(field), f"sandbox_profile.{field}")
        except ValueError as err:
            errors.append(str(err))

    for field in ("completion_contract", "non_goals", "north_star_components"):
        try:
            require_list(spec.get(field), field)
        except ValueError as err:
            errors.append(str(err))

    for rule_file in REQUIRED_RULE_FILES:
        if not (repo_root / rule_file).exists():
            errors.append(f"missing PADS rule file: {rule_file}")

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate Solar PADS cross-references")
    parser.add_argument("--json", dest="json_path", default=".pads/spec.json")
    parser.add_argument("--repo-root", default=".")
    args = parser.parse_args()

    repo_root = Path(args.repo_root)
    try:
        spec = load_json(Path(args.json_path))
        errors = validate(spec, repo_root)
    except ValueError as err:
        sys.stderr.write(f"[pads/validate] {err}\n")
        return 2

    if errors:
        sys.stderr.write("[pads/validate] structural validation failed:\n")
        for error in errors:
            sys.stderr.write(f"  - {error}\n")
        return 1

    print("[pads/validate] OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
