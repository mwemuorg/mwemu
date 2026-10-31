#!/usr/bin/env bash
# Complexity gate: fails if the number of functions over the CC threshold
# grows beyond the grandfathered baseline. Run: make complexity-gate
#
# To update the baseline after a justified refactor:
#   python3 scripts/complexity.py --all --threshold 40 2>&1 | grep "Over threshold"
#   # update BASELINE below with the new count

set -uo pipefail
cd "$(git rev-parse --show-toplevel)"

THRESHOLD=40
BASELINE=42   # grandfathered count as of 2026-10-26
MAX_CC=900
MAX_LINES=1500

OUTPUT=$(python3 scripts/complexity.py --all --threshold "$THRESHOLD" \
    --max-cc "$MAX_CC" --max-lines "$MAX_LINES" 2>&1 || true)

# Hard ceiling check (exit code 2 from complexity.py)
if echo "$OUTPUT" | grep -q "HARD CEILING VIOLATED"; then
    echo "$OUTPUT" | grep -A100 "HARD CEILING"
    exit 1
fi

COUNT=$(echo "$OUTPUT" | grep "Over threshold" | grep -oE '[0-9]+$')

if [ -z "$COUNT" ]; then
    echo "ERROR: could not parse complexity output"
    echo "$OUTPUT"
    exit 2
fi

echo "Functions over CC $THRESHOLD: $COUNT (baseline: $BASELINE)"

if [ "$COUNT" -gt "$BASELINE" ]; then
    echo "FAIL: complexity grew from $BASELINE to $COUNT"
    echo "New offenders:"
    python3 scripts/complexity.py --threshold "$THRESHOLD" --top 60 2>&1 | grep '!!!' || true
    exit 1
fi

if [ "$COUNT" -lt "$BASELINE" ]; then
    echo "NOTE: complexity dropped to $COUNT — consider updating BASELINE in $0"
fi

echo "OK"
