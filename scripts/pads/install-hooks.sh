#!/usr/bin/env bash
# Install a local pre-commit hook that runs the Tier-0 and append-only
# guards. Idempotent; safe to run repeatedly.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
HOOKS_DIR="$REPO_ROOT/.git/hooks"
HOOK_FILE="$HOOKS_DIR/pre-commit"

if [ ! -d "$HOOKS_DIR" ]; then
  echo "[pads:install-hooks] $HOOKS_DIR does not exist — not a git checkout?" >&2
  exit 1
fi

cat > "$HOOK_FILE" <<'HOOK'
#!/usr/bin/env bash
# Installed by scripts/pads/install-hooks.sh; safe to remove with `rm`.
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "$REPO_ROOT"

if ! command -v python3 >/dev/null 2>&1; then
  echo "[pre-commit] python3 not available; skipping PADS guards." >&2
  exit 0
fi

if [ -f PADS.md ] && [ -f .pads/tier0.sha256 ]; then
  python3 scripts/pads/tier0-guard.py --spec PADS.md --expected .pads/tier0.sha256 \
    || { echo "[pre-commit] Tier-0 guard failed; see above." >&2; exit 1; }
fi

python3 scripts/pads/append-only-guard.py \
  || { echo "[pre-commit] Append-only guard failed; see above." >&2; exit 1; }
HOOK
chmod +x "$HOOK_FILE"

echo "[pads:install-hooks] Installed $HOOK_FILE"
