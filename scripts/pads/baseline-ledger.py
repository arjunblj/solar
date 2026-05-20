#!/usr/bin/env python3
"""Emit a deterministic local baseline ledger for PADS tasks.

The ledger is intentionally infrastructure-only: it records which local tools,
corpora, and repository commands are available without claiming compiler
compatibility or runtime correctness.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
from pathlib import Path
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OUTPUT = ROOT / ".pads-artifacts" / "baseline.json"

TOOL_VERSION_COMMANDS: tuple[tuple[str, tuple[str, ...]], ...] = (
    ("python3", ("python3", "--version")),
    ("cargo", ("cargo", "--version")),
    ("rustc", ("rustc", "--version")),
    ("solc", ("solc", "--version")),
    ("forge", ("forge", "--version")),
    ("cargo-nextest", ("cargo", "nextest", "--version")),
    ("jq", ("jq", "--version")),
)

COMMANDS: tuple[dict[str, Any], ...] = (
    {
        "name": "cargo-check-workspace",
        "argv": ("cargo", "check", "--workspace"),
        "required_tools": ("cargo",),
    },
    {
        "name": "cargo-nextest-workspace",
        "argv": ("cargo", "nextest", "run", "--workspace"),
        "required_tools": ("cargo", "cargo-nextest"),
    },
    {
        "name": "solc-version",
        "argv": ("solc", "--version"),
        "required_tools": ("solc",),
    },
    {
        "name": "forge-version",
        "argv": ("forge", "--version"),
        "required_tools": ("forge",),
    },
    {
        "name": "tester-ui-mode",
        "argv": ("cargo", "test", "-p", "solar-compiler", "--test", "tests", "--", "--list"),
        "required_tools": ("cargo",),
        "env": {"TESTER_MODE": "ui"},
    },
    {
        "name": "tester-solc-solidity-mode",
        "argv": ("cargo", "test", "-p", "solar-compiler", "--test", "tests", "--", "--list"),
        "required_tools": ("cargo",),
        "required_corpora": ("solc-solidity",),
        "env": {"TESTER_MODE": "solc-solidity"},
    },
    {
        "name": "tester-solc-solidity-typeck-mode",
        "argv": ("cargo", "test", "-p", "solar-compiler", "--test", "tests", "--", "--list"),
        "required_tools": ("cargo",),
        "required_corpora": ("solc-solidity",),
        "env": {"TESTER_MODE": "solc-solidity-typeck"},
    },
    {
        "name": "tester-solc-yul-mode",
        "argv": ("cargo", "test", "-p", "solar-compiler", "--test", "tests", "--", "--list"),
        "required_tools": ("cargo",),
        "required_corpora": ("solc-yul",),
        "env": {"TESTER_MODE": "solc-yul"},
    },
    {
        "name": "intentional-typo-probe",
        "argv": ("solcc", "--version"),
        "required_tools": ("solcc",),
    },
)

CORPORA: tuple[dict[str, Any], ...] = (
    {"name": "ui", "paths": ("tests/ui", "crates/solar/tests/ui"), "patterns": ("*.sol", "*.yul")},
    {"name": "solc-solidity", "paths": ("testdata/solidity/test",), "patterns": ("*.sol",)},
    {"name": "solc-yul", "paths": ("testdata/solidity/test/libyul",), "patterns": ("*.yul",)},
    {"name": "foundry", "paths": ("testdata/foundry", "corpus/foundry", "foundry")},
)

TESTER_MODES: tuple[dict[str, str], ...] = (
    {"name": "ui", "env": "TESTER_MODE=ui", "source": "tools/tester/src/lib.rs"},
    {"name": "solc-solidity", "env": "TESTER_MODE=solc-solidity", "source": "tools/tester/src/lib.rs"},
    {
        "name": "solc-solidity-typeck",
        "env": "TESTER_MODE=solc-solidity-typeck",
        "source": "tools/tester/src/lib.rs",
    },
    {"name": "solc-yul", "env": "TESTER_MODE=solc-yul", "source": "tools/tester/src/lib.rs"},
)

TESTER_MODE_CORPORA: dict[str, str] = {
    "ui": "ui",
    "solc-solidity": "solc-solidity",
    "solc-solidity-typeck": "solc-solidity",
    "solc-yul": "solc-yul",
}


class CommandResult(dict[str, Any]):
    """Typed marker for command result dictionaries."""


def rel(path: Path) -> str:
    try:
        return path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        return path.as_posix()


def run(argv: Iterable[str], *, env: dict[str, str] | None = None, timeout: float = 60.0) -> CommandResult:
    argv_tuple = tuple(argv)
    try:
        completed = subprocess.run(
            argv_tuple,
            cwd=ROOT,
            env={**os.environ, **(env or {})},
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
            check=False,
        )
        return CommandResult(
            status="ok" if completed.returncode == 0 else "failed",
            exit_code=completed.returncode,
            stdout=completed.stdout.strip().splitlines()[:20],
            stderr=completed.stderr.strip().splitlines()[:20],
        )
    except FileNotFoundError as exc:
        return CommandResult(
            status="unavailable",
            exit_code=None,
            error=str(exc),
        )
    except subprocess.TimeoutExpired as exc:
        return CommandResult(
            status="timeout",
            exit_code=None,
            stdout=(exc.stdout or "").strip().splitlines()[:20] if isinstance(exc.stdout, str) else [],
            stderr=(exc.stderr or "").strip().splitlines()[:20] if isinstance(exc.stderr, str) else [],
        )


def command_status(name: str, result: CommandResult) -> CommandResult:
    out = CommandResult(name=name)
    out.update(result)
    return out


def repo_info() -> dict[str, Any]:
    sha_result = run(("git", "rev-parse", "HEAD"), timeout=10.0)
    branch_result = run(("git", "rev-parse", "--abbrev-ref", "HEAD"), timeout=10.0)
    status_result = run(("git", "status", "--porcelain"), timeout=10.0)
    sha = sha_result.get("stdout", [None])[0] if sha_result.get("status") == "ok" else None
    branch = branch_result.get("stdout", [None])[0] if branch_result.get("status") == "ok" else None
    dirty = bool(status_result.get("stdout")) if status_result.get("status") == "ok" else None
    return {
        "root": rel(ROOT),
        "sha": sha,
        "branch": branch,
        "dirty": dirty,
        "git": {
            "sha_status": sha_result["status"],
            "branch_status": branch_result["status"],
            "status_status": status_result["status"],
        },
    }


def tool_available(tool_name: str, result: CommandResult) -> bool:
    if tool_name == "cargo-nextest":
        return result["status"] == "ok"
    executable = tool_name
    if tool_name == "cargo-nextest":
        executable = "cargo"
    return shutil.which(executable) is not None and result["status"] == "ok"


def collect_tools() -> dict[str, Any]:
    tools: dict[str, Any] = {}
    for name, argv in TOOL_VERSION_COMMANDS:
        result = run(argv, timeout=20.0)
        version_lines = result.get("stdout") or result.get("stderr") or []
        tools[name] = {
            "status": result["status"],
            "available": tool_available(name, result),
            "version": version_lines[0] if version_lines else None,
            "argv": list(argv),
        }
        if result.get("exit_code") is not None:
            tools[name]["exit_code"] = result["exit_code"]
        if result.get("error"):
            tools[name]["error"] = result["error"]
    return tools


def collect_commands(tools: dict[str, Any], corpora_by_name: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    commands: list[dict[str, Any]] = []
    for spec in COMMANDS:
        missing = [name for name in spec["required_tools"] if tools.get(name, {}).get("status") != "ok"]
        missing_corpora = [
            name for name in spec.get("required_corpora", ()) if corpora_by_name.get(name, {}).get("status") != "available"
        ]
        entry: dict[str, Any] = {
            "name": spec["name"],
            "argv": list(spec["argv"]),
            "env": spec.get("env", {}),
        }
        if missing or missing_corpora:
            entry.update(
                {
                    "status": "unavailable",
                    "exit_code": None,
                    "missing_tools": missing,
                    "missing_corpora": missing_corpora,
                }
            )
        else:
            result = run(spec["argv"], env=spec.get("env"), timeout=120.0)
            entry["status"] = result["status"]
            entry["exit_code"] = result["exit_code"]
            if result.get("error"):
                entry["error"] = result["error"]
        commands.append(entry)
    return commands


def count_files(path: Path, patterns: tuple[str, ...]) -> int:
    if path.is_file():
        return 1
    return sum(1 for child in path.rglob("*") if child.is_file() and matches_patterns(child, patterns))


def matches_patterns(path: Path, patterns: tuple[str, ...]) -> bool:
    return not patterns or any(path.match(pattern) for pattern in patterns)


def collect_corpora() -> list[dict[str, Any]]:
    corpora: list[dict[str, Any]] = []
    for spec in CORPORA:
        candidates = [ROOT / path for path in spec["paths"]]
        existing = [path for path in candidates if path.exists()]
        file_count = 0
        patterns = tuple(spec.get("patterns", ()))
        if existing:
            for path in existing:
                file_count += count_files(path, patterns)
        corpora.append(
            {
                "name": spec["name"],
                "status": "available" if existing else "unavailable",
                "paths": [rel(path) for path in candidates],
                "available_paths": [rel(path) for path in existing],
                "file_count": file_count,
                "patterns": list(patterns),
            }
        )
    return corpora


def collect_tester_lanes(tools: dict[str, Any], corpora_by_name: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    lanes: list[dict[str, Any]] = []
    for mode in TESTER_MODES:
        name = mode["name"]
        corpus_name = TESTER_MODE_CORPORA[name]
        corpus = corpora_by_name[corpus_name]
        missing_tools = []
        if name.startswith("solc-") and tools.get("solc", {}).get("status") != "ok":
            missing_tools.append("solc")

        status = "available"
        if missing_tools or corpus["status"] != "available":
            status = "unavailable"

        lanes.append(
            {
                "name": name,
                "env": mode["env"],
                "source": mode["source"],
                "corpus": corpus_name,
                "status": status,
                "missing_tools": missing_tools,
                "missing_corpora": [] if corpus["status"] == "available" else [corpus_name],
                "counts": {
                    "total": corpus["file_count"],
                    "available": corpus["file_count"] if status == "available" else 0,
                    "unavailable": 0 if status == "available" else corpus["file_count"],
                    "pass": None,
                    "fail": None,
                    "skip": 0,
                    "xfail": 0,
                },
            }
        )
    return lanes


def build_ledger() -> dict[str, Any]:
    tools = collect_tools()
    corpora = collect_corpora()
    corpora_by_name = {corpus["name"]: corpus for corpus in corpora}
    return {
        "schema_version": 1,
        "repo": repo_info(),
        "tools": tools,
        "tester_modes": collect_tester_lanes(tools, corpora_by_name),
        "corpora": corpora,
        "commands": collect_commands(tools, corpora_by_name),
    }


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    encoded = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    path.write_text(encoded, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT, help="ledger path to write")
    parser.add_argument("--json", action="store_true", help="write ledger JSON to stdout instead of a file")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    ledger = build_ledger()
    if args.json:
        print(json.dumps(ledger, indent=2, sort_keys=True))
        return 0
    output = args.output if args.output.is_absolute() else ROOT / args.output
    write_json(output, ledger)
    print(rel(output))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())