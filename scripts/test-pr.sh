#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <PR_NUMBER> [cargo test extra args...]"
    exit 1
fi

PR="$1"
shift
WORKTREE="/tmp/mwemu-pr-${PR}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [ -d "$WORKTREE" ]; then
    echo "Worktree $WORKTREE already exists. Run scripts/cleanup-pr.sh $PR first."
    exit 1
fi

echo "==> Fetching PR #${PR}..."
git -C "$REPO_ROOT" fetch origin "pull/${PR}/head:pr-${PR}"

echo "==> Creating worktree at ${WORKTREE}..."
git -C "$REPO_ROOT" worktree add "$WORKTREE" "pr-${PR}"

echo "==> Linking maps and test data..."
[ -d "$REPO_ROOT/maps" ] && ln -s "$REPO_ROOT/maps" "$WORKTREE/maps" 2>/dev/null || true
[ -d "$REPO_ROOT/test" ] && ln -s "$REPO_ROOT/test" "$WORKTREE/test" 2>/dev/null || true

echo "==> Running cargo test..."
TARGET=""
if [ "$(uname -m)" = "arm64" ]; then
    TARGET="--target x86_64-apple-darwin"
fi

cd "$WORKTREE"
cargo test $TARGET "$@"
EXIT=$?

echo ""
echo "==> Tests finished (exit $EXIT). Worktree at: $WORKTREE"
echo "    cd $WORKTREE"
echo "    Clean up with: $REPO_ROOT/scripts/cleanup-pr.sh $PR"
cd "$WORKTREE"
exec "$SHELL"
