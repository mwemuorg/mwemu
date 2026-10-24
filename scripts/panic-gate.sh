#!/usr/bin/env bash
# Panic-count regression gate for guest-reachable code paths.
# Fails if .unwrap()/.expect()/panic! counts in engine/, emu/, maps/
# exceed the recorded baseline. Run from the repo root.

SRC="crates/libmwemu/src"

# Baseline — update these when you intentionally reduce the counts.
declare -A BASELINE=(
  [engine_unwrap]=4
  [engine_expect]=0
  [engine_panic]=0
  [emu_unwrap]=31
  [emu_expect]=6
  [emu_panic]=5
  [maps_unwrap]=13
  [maps_expect]=26
  [maps_panic]=0
)

count_matches() {
  local pattern="$1" dir="$2"
  local result
  result=$(grep -rc "$pattern" "$dir" --include="*.rs" 2>/dev/null | awk -F: '{sum += $2} END {print sum+0}')
  echo "${result:-0}"
}

fail=0
for area in engine emu maps; do
  for sig_key in unwrap expect panic; do
    case "$sig_key" in
      unwrap) pattern='.unwrap()' ;;
      expect) pattern='.expect('  ;;
      panic)  pattern='panic!'    ;;
    esac

    key="${area}_${sig_key}"
    count=$(count_matches "$pattern" "$SRC/$area/")
    max=${BASELINE[$key]}

    if [ "$count" -gt "$max" ]; then
      echo "FAIL: $pattern in $area/ = $count (baseline $max)"
      fail=1
    elif [ "$count" -lt "$max" ]; then
      echo "IMPROVED: $pattern in $area/ = $count (baseline was $max — please update scripts/panic-gate.sh)"
    else
      echo "OK: $pattern in $area/ = $count (baseline $max)"
    fi
  done
done

exit $fail
