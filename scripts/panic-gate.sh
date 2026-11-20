#!/usr/bin/env bash
# Panic-count regression gate for guest-reachable code paths.
# Fails if .unwrap()/.expect()/panic!/assert! counts in engine/, emu/, maps/
# exceed the recorded baseline. Run from the repo root. (`assert!` aborts in
# release builds too, so it counts; `debug_assert!` does not.)
# Only production code is counted — lines inside #[cfg(test)] modules and
# files under tests/ directories are excluded.

SRC="crates/libmwemu/src"

# Baseline — update these when you intentionally reduce the counts.
declare -A BASELINE=(
  [engine_unwrap]=2
  [engine_expect]=0
  [engine_panic]=0
  [emu_unwrap]=5
  [emu_expect]=0
  [emu_panic]=5
  [maps_unwrap]=13
  [maps_expect]=9
  [maps_panic]=0
  [engine_assert]=0
  [emu_assert]=17
  [maps_assert]=1
)

# Strip test blocks from a Rust file: removes everything from a line
# containing #[cfg(test)] to the end of the file (the test module is
# conventionally the last item), and also drops #[test] functions.
# This is a rough heuristic that works for the codebase's layout.
strip_test_blocks() {
  sed '/#\[cfg(test)\]/,$d'
}

# Count pattern matches in production code only.
count_matches() {
  local pattern="$1" dir="$2"
  local result=0
  while IFS= read -r -d '' file; do
    local n
    n=$(strip_test_blocks < "$file" | grep -c "$pattern" 2>/dev/null || true)
    result=$((result + n))
  done < <(find "$dir" -name '*.rs' ! -path '*/tests/*' -print0 2>/dev/null)
  echo "$result"
}

# assert!/assert_eq!/assert_ne! not preceded by `debug_` (extended regex).
count_asserts() {
  local dir="$1"
  local result=0
  while IFS= read -r -d '' file; do
    local n
    n=$(strip_test_blocks < "$file" | grep -cE '(^|[^_a-z])assert(_eq|_ne)?!\(' 2>/dev/null || true)
    result=$((result + n))
  done < <(find "$dir" -name '*.rs' ! -path '*/tests/*' -print0 2>/dev/null)
  echo "$result"
}

fail=0
for area in engine emu maps; do
  for sig_key in unwrap expect panic assert; do
    case "$sig_key" in
      unwrap) pattern='.unwrap()' ;;
      expect) pattern='.expect('  ;;
      panic)  pattern='panic!'    ;;
      assert) pattern='assert!'   ;;
    esac

    key="${area}_${sig_key}"
    if [ "$sig_key" = assert ]; then
      count=$(count_asserts "$SRC/$area/")
    else
      count=$(count_matches "$pattern" "$SRC/$area/")
    fi
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
