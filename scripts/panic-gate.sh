#!/usr/bin/env bash
# Panic-count regression gate for guest-reachable code paths.
# Fails if .unwrap()/.expect()/panic!/assert! counts in engine/, emu/, maps/
# exceed the recorded baseline. Run from the repo root. (`assert!` aborts in
# release builds too, so it counts; `debug_assert!` does not.)

# Where each gated area lives. They were folders of one crate before the
# crate split (#236); the counts follow the code.
declare -A DIRS=(
  [engine]="crates/mwemu-x86/src/engine crates/mwemu-aarch64/src/engine"
  [emu]="crates/mwemu-core/src/emu"
  [maps]="crates/mwemu-core/src/maps"
)

# Baseline — update these when you intentionally reduce the counts.
# emu_unwrap and emu_assert went up by 12 and 8 when loaders/macho/macho64.rs
# moved to emu/loaders/ (#228): the Mach-O parser is now under this gate too.
declare -A BASELINE=(
  [engine_unwrap]=2
  [engine_expect]=0
  [engine_panic]=0
  [emu_unwrap]=44
  [emu_expect]=6
  [emu_panic]=5
  [maps_unwrap]=13
  [maps_expect]=26
  [maps_panic]=0
  [engine_assert]=0
  [emu_assert]=97
  [maps_assert]=39
)

count_matches() {
  local pattern="$1"
  shift
  local result
  result=$(grep -rc "$pattern" "$@" --include="*.rs" 2>/dev/null | awk -F: '{sum += $2} END {print sum+0}')
  echo "${result:-0}"
}

# assert!/assert_eq!/assert_ne! not preceded by `debug_` (extended regex).
count_asserts() {
  local result
  result=$(grep -rcE '(^|[^_a-z])assert(_eq|_ne)?!\(' "$@" --include="*.rs" 2>/dev/null | awk -F: '{sum += $2} END {print sum+0}')
  echo "${result:-0}"
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
      # shellcheck disable=SC2086  # one area can be several directories
      count=$(count_asserts ${DIRS[$area]})
    else
      # shellcheck disable=SC2086
      count=$(count_matches "$pattern" ${DIRS[$area]})
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
