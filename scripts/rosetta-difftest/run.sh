#!/bin/sh
# Differential test of libmwemu against the real CPU, through Rosetta.
# macOS on Apple Silicon only: x86-64 test snippets run natively under Rosetta
# (the oracle) and in pymwemu, and registers, memory and the architecturally
# defined flags are compared. Needs the pymwemu venv (make -C crates/pymwemu
# setup-env) rebuilt after any libmwemu change (maturin develop --release).
set -e
cd "$(dirname "$0")"
PY=${PYMWEMU_PYTHON:-../../crates/pymwemu/.env/bin/python}
if [ ! -x "$PY" ]; then
    echo "pymwemu venv not found at $PY; run 'make -C crates/pymwemu setup-env'" >&2
    exit 1
fi
if ! arch -x86_64 /usr/bin/true 2>/dev/null; then
    echo "Rosetta is required (softwareupdate --install-rosetta)" >&2
    exit 1
fi
clang -arch x86_64 -O1 -Wall oracle.c -o oracle
clang -arch x86_64 -O1 -Wall oracle2.c -o oracle2
echo "== register forms =="
"$PY" -I difftest.py "$@"
echo
echo "== memory forms and string instructions =="
"$PY" -I difftest_mem.py "$@"
