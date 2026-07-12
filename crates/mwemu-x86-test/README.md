# mwemu-x86-test

Instruction-semantics test harness for **libmwemu**. It replays hardware-oracle
x86-64 test vectors through the emulator one instruction at a time and diffs the
resulting CPU state against the oracle.

## What it is (and what it is not)

- It drives **libmwemu directly** (in-process, native Rust): build a clean
  `emu64()`, set the initial registers/flags/XMM/memory, map the code page,
  `step()` once, read the state back, compare.
- It **does not** use Remill, XED, LLVM, or any C++ component. Remill is tested
  separately by the `remill-tester` project. Both harnesses consume the *same*
  corpus so their results are comparable, but this crate never launches Remill.
- The oracle is **real hardware**: the corpus was produced by running each
  instruction encoding on physical CPUs (x86Tester / binit) and recording the
  before/after state. mwemu is compared against that ground truth, not against
  Remill.

```
corpus .txt  ──▶  corpus::parse  ──▶  driver::run_row(libmwemu::emu64())  ──▶  diff vs oracle
(hardware oracle)                     set state · step() · read state
```

## Corpus format

The current pooled x86Tester text format (a shared value pool + `instr:` headers
+ index rows). Convert parquet datasets to it with the scripts in
`remill-tester/tools/` (`convert_test_vectors.py`, `convert_public_parquet.py`);
the format is emulator-agnostic.

## Usage

```bash
# one file
cargo run -p mwemu-x86-test --release -- path/to/corpus.txt

# a directory, only some mnemonics, capped
cargo run -p mwemu-x86-test --release -- \
    --input-dir testdata/corpus --mnemonic adc,add,xor --limit 50000

# stop at the first mismatch
cargo run -p mwemu-x86-test --release -- corpus.txt --stop-on-first-fail
```

Exit code is non-zero when any selected row mismatches. The summary buckets rows
into `pass / fail / unsupported / not-executed`.

## Known limitations / next steps

1. **`not-executed` vs `unimplemented` are not distinguished.** libmwemu's
   `step()` returns a single `bool`; an unimplemented instruction, a benign
   `false`, and some fault paths all look alike. The clean fix is a small
   libmwemu addition — a `test_step()` (or `StepOutcome { Executed, Unimplemented,
   Fault(kind), DecodeError }`) — so this harness can bucket coverage correctly
   instead of lumping them under `not-executed`.
2. **Undefined-flag masking is coarse.** Only the six arithmetic status flags
   (CF/PF/AF/ZF/SF/OF) are compared; flags a given instruction leaves undefined
   are not yet masked per-instruction. libmwemu already depends on `iced-x86`,
   whose `InstructionInfoFactory` exposes `rflags_undefined` — wiring that in is
   the intended next step. Use `--no-flags` meanwhile to silence spurious flag
   diffs.
3. **Not modelled yet:** x87 (`st<n>`, `x87status`), MMX (`mm<n>`), AVX-512
   (`zmm<n>`), YMM outputs, and control/debug registers. Rows referencing these
   are reported as `unsupported` skips.
4. **Exception oracle is coarse.** A row expecting an exception passes if
   libmwemu faults at all; the specific CPU vector is not yet matched.
