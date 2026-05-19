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
import time
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
        "argv": ("cargo", "test", "-p", "solar", "--test", "tests", "--", "--list"),
        "required_tools": ("cargo",),
        "env": {"TESTER_MODE": "ui"},
    },
    {
        "name": "tester-solc-solidity-mode",
        "argv": ("cargo", "test", "-p", "solar", "--test", "tests", "--", "--list"),
        "required_tools": ("cargo",),
        "env": {"TESTER_MODE": "solc-solidity"},
    },
    {
        "name": "tester-solc-yul-mode",
        "argv": ("cargo", "test", "-p", "solar", "--test", "tests", "--", "--list"),
        "required_tools": ("cargo",),
        "env": {"TESTER_MODE": "solc-yul"},
    },
    {
        "name": "intentional-typo-probe",
        "argv": ("solcc", "--version"),
        "required_tools": ("solcc",),
    },
)

CORPORA: tuple[dict[str, Any], ...] = (
    {"name": "ui", "paths": ("crates/solar/tests/ui", "tests/ui")},
    {"name": "solc-solidity", "paths": ("testdata/solidity", "tests/solc/solidity", "crates/solar/tests/solc")},
    {"name": "solc-yul", "paths": ("testdata/yul", "tests/solc/yul", "crates/solar/tests/yul")},
    {"name": "foundry", "paths": ("testdata/foundry", "corpus/foundry", "foundry")},
)

TESTER_MODES: tuple[dict[str, str], ...] = (
    {"name": "ui", "env": "TESTER_MODE=ui", "source": "tools/tester/src/lib.rs"},
    {"name": "solc-solidity", "env": "TESTER_MODE=solc-solidity", "source": "tools/tester/src/lib.rs"},
    {"name": "solc-yul", "env": "TESTER_MODE=solc-yul", "source": "tools/tester/src/lib.rs"},
)


class CommandResult(dict[str, Any]):
    """Typed marker for command result dictionaries."""


def rel(path: Path) -> str:
    try:
        return path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        return path.as_posix()


def run(argv: Iterable[str], *, env: dict[str, str] | None = None, timeout: float = 60.0) -> CommandResult:
    argv_tuple = tuple(argv)
    started = time.monotonic()
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
        elapsed = time.monotonic() - started
        return CommandResult(
            status="ok" if completed.returncode == 0 else "failed",
            exit_code=completed.returncode,
            elapsed_seconds=round(elapsed, 3),
            stdout=completed.stdout.strip().splitlines()[:20],
            stderr=completed.stderr.strip().splitlines()[:20],
        )
    except FileNotFoundError as exc:
        elapsed = time.monotonic() - started
        return CommandResult(
            status="unavailable",
            exit_code=None,
            elapsed_seconds=round(elapsed, 3),
            error=str(exc),
        )
    except subprocess.TimeoutExpired as exc:
        elapsed = time.monotonic() - started
        return CommandResult(
            status="timeout",
            exit_code=None,
            elapsed_seconds=round(elapsed, 3),
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
            "elapsed_seconds": result["elapsed_seconds"],
        }
        if result.get("exit_code") is not None:
            tools[name]["exit_code"] = result["exit_code"]
        if result.get("error"):
            tools[name]["error"] = result["error"]
    return tools


def collect_commands(tools: dict[str, Any]) -> list[dict[str, Any]]:
    commands: list[dict[str, Any]] = []
    for spec in COMMANDS:
        missing = [name for name in spec["required_tools"] if tools.get(name, {}).get("status") != "ok"]
        entry: dict[str, Any] = {
            "name": spec["name"],
            "argv": list(spec["argv"]),
            "env": spec.get("env", {}),
        }
        if missing:
            entry.update(
                {
                    "status": "unavailable",
                    "exit_code": None,
                    "elapsed_seconds": 0.0,
                    "missing_tools": missing,
                }
            )
        else:
            entry.update(run(spec["argv"], env=spec.get("env"), timeout=120.0))
        commands.append(entry)
    return commands


def collect_corpora() -> list[dict[str, Any]]:
    corpora: list[dict[str, Any]] = []
    for spec in CORPORA:
        candidates = [ROOT / path for path in spec["paths"]]
        existing = [path for path in candidates if path.exists()]
        file_count = 0
        if existing:
            for path in existing:
                if path.is_file():
                    file_count += 1
                else:
                    file_count += sum(1 for child in path.rglob("*") if child.is_file())
        corpora.append(
            {
                "name": spec["name"],
                "status": "available" if existing else "unavailable",
                "paths": [rel(path) for path in candidates],
                "available_paths": [rel(path) for path in existing],
                "file_count": file_count,
            }
        )
    return corpora


def build_ledger() -> dict[str, Any]:
    tools = collect_tools()
    return {
        "schema_version": 1,
        "repo": repo_info(),
        "tools": tools,
        "tester_modes": list(TESTER_MODES),
        "corpora": collect_corpora(),
        "commands": collect_commands(tools),
    }


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    encoded = json.dumps(payload, indent=2, sort_keys=True) + "\n"
    path.write_text(encoded, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT, help="ledger path to write")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    output = args.output if args.output.is_absolute() else ROOT / args.output
    write_json(output, build_ledger())
    print(rel(output))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())