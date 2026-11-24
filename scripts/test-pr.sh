#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <PR_NUMBER> [cargo nextest arguments...]"
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

echo "==> Copying maps and test data..."
if [ -d "$REPO_ROOT/maps" ] && [[ "$WORKTREE" == /tmp/mwemu-pr-* ]]; then
    rm -rf "$WORKTREE/maps"
    cp -r "$REPO_ROOT/maps" "$WORKTREE/maps"
fi
if [ -d "$REPO_ROOT/test" ] && [[ "$WORKTREE" == /tmp/mwemu-pr-* ]]; then
    rm -rf "$WORKTREE/test"
    cp -r "$REPO_ROOT/test" "$WORKTREE/test"
fi

echo "==> Running cargo nextest..."
cd "$WORKTREE"
mkdir -p .config
cat > .config/nextest.toml <<'TOML'
[profile.default]
slow-timeout = { period = "60s", terminate-after = 1 }
TOML
# Older PRs predate the asset-tests feature. Respect explicit feature and package selections.
auto_features=true
auto_release=true
for arg in "$@"; do
    case "$arg" in
        --) break ;;
        -F*|--features|--features=*|--no-default-features|--all-features|-p*|--package|--package=*|--exclude|--exclude=*) auto_features=false ;;
        --release|--cargo-profile|--cargo-profile=*) auto_release=false ;;
    esac
done
if [[ "$auto_features" == true ]]; then
    if [[ -n "${PYTHON:-}" ]]; then
        python_cmd=$PYTHON
    elif command -v python3 >/dev/null; then
        python_cmd=python3
    else
        python_cmd=python
    fi
    feature=$(cargo metadata --format-version 1 --no-deps | "$python_cmd" -c '
import json, sys
sys.stdout.write(next(("libmwemu/asset-tests" for p in json.load(sys.stdin)["packages"]
            if p["name"] == "libmwemu" and "asset-tests" in p["features"]), ""))
')
    if [[ -n "$feature" ]]; then
        set -- --features "$feature" "$@"
        if [[ "$auto_release" == true ]]; then
            set -- --release "$@"
        fi
    fi
fi
cargo nextest run "$@"
EXIT=$?

echo ""
echo "==> Tests finished (exit $EXIT). Worktree at: $WORKTREE"
echo "    cd $WORKTREE"
echo "    Clean up with: $REPO_ROOT/scripts/cleanup-pr.sh $PR"
cd "$WORKTREE"
exec "$SHELL"
