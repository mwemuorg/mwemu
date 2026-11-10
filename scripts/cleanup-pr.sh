#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <PR_NUMBER>"
    exit 1
fi

PR="$1"
WORKTREE="/tmp/mwemu-pr-${PR}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [ ! -d "$WORKTREE" ]; then
    echo "No worktree at $WORKTREE"
else
    echo "==> Removing worktree ${WORKTREE}..."
    git -C "$REPO_ROOT" worktree remove --force "$WORKTREE"
fi

if git -C "$REPO_ROOT" rev-parse --verify "pr-${PR}" >/dev/null 2>&1; then
    echo "==> Deleting branch pr-${PR}..."
    git -C "$REPO_ROOT" branch -D "pr-${PR}"
fi

echo "Done."
