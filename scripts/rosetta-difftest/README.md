# Rosetta differential test

Diffs libmwemu's x86-64 integer semantics against a real CPU without any x86
hardware: on Apple Silicon, Rosetta runs x86-64 code with hardware-faithful
flag behaviour, so it serves as the oracle.

```
scripts/rosetta-difftest/run.sh                # ~10k random vectors, two sweeps
scripts/rosetta-difftest/run.sh --seed 7 --n 8 # more states per instruction
scripts/rosetta-difftest/run.sh --fam shift    # one family (see the catalogs)
```

How it works:

- `asmlib.py` assembles Intel-syntax snippets with `clang -arch x86_64` and
  reads the bytes back with `objdump`.
- Every vector is `push imm32 ; popfq ; <insn> ; pushfq ; pop r11`, so the
  input flags are set and the output flags land in `r11`.
- `oracle.c` / `oracle2.c` copy a trampoline plus the vector into an RWX page
  and run it (`oracle2` also maps a 64 KB buffer at `0x300000000` for the memory
  and string forms). `SIGFPE`/`SIGILL`/`SIGSEGV` are reported as `EXC`.
- `difftest.py` (register forms) and `difftest_mem.py` (memory forms, string
  instructions, `bt*` with register offsets, `cmpxchg8b/16b`, `xlatb`,
  `crc32`...) run the same bytes in pymwemu and compare registers, memory and
  the flags each instruction defines (undefined flags are masked per the SDM).

Requirements: macOS on Apple Silicon with Rosetta, Xcode command line tools,
and the pymwemu venv (`make -C crates/pymwemu setup-env`). Rebuild pymwemu
after changing libmwemu (`maturin develop --release` inside the venv) or the
sweep tests stale code.

Known oracle quirks, not libmwemu bugs:

- `cmpxchg`: Rosetta derives CF/SF/AF/PF from `dest - acc`; real silicon (and
  libmwemu, verified against the x86Tester corpus) use `acc - dest`. Only the
  flags differ, so `cmpxchg` always shows flag mismatches.
- `adcx`/`adox` raise SIGILL under Rosetta (no ADX), reported as `oracle EXC`.
- `shld`/`shrd` with 16-bit operands and a count above 16 are undefined and
  the memory result differs.

## Recorded vectors for CI

`gen_vectors.py` records one deterministic state per catalog instruction from
the real CPU into `crates/libmwemu/src/tests/isa/x64/cpu_vectors.rs`
(344 register and memory vectors, with the defined-flag masks), which
`cpu_vector_tests.rs` replays in libmwemu on every platform. Regenerate it
after extending the catalogs:

```
python3 -I scripts/rosetta-difftest/gen_vectors.py
```
