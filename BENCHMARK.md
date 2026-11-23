
# BENCHMARK

Run `python scripts/sync_assets.py` to prepare the inputs. `cargo bench-pe` times loading the Enigma sample and emulating 20 million guest instructions with the optimized Cargo bench profile. It uses the native target by default. On Apple Silicon, use `cargo bench-pe --target x86_64-apple-darwin` to measure an Intel build under Rosetta. The older measurements below include CLI and Cargo startup.

## Current (v1.0.1)

### Apple M4, 16 GB RAM, macOS

**Native arm64:**
```
real    0m1.530s
user    0m1.068s
sys     0m0.027s
[speed] 20,000,000 instructions emulated successfully.
```
- Wall-clock: **~13,000,000 instructions/second**

**Rosetta 2 (x86_64-apple-darwin):**
```
real    0m3.402s
user    0m1.690s
sys     0m0.100s
[speed] 20,000,000 instructions emulated successfully.
```
- Wall-clock: **~5,900,000 instructions/second** (~2.2x slower than native)

## Historical (v0.21.4)

```
AMD Ryzen 9 8945HS w/ Radeon 780M Graphics
64 GB RAM, Linux

10M instructions, 5 runs each scenario
```

### 32bits no verbose (10M instructions)

| Run | Wall-clock |
|-----|-----------|
| 1   | 508 ms    |
| 2   | 500 ms    |
| 3   | 513 ms    |
| 4   | 511 ms    |
| 5   | 528 ms    |

**~19,600,000 instructions/second**

### 32bits with verbose -vv (10M instructions)

| Run | Wall-clock |
|-----|-----------|
| 1   | 17.33 s   |
| 2   | 17.21 s   |
| 3   | 16.92 s   |
| 4   | 17.48 s   |
| 5   | 17.41 s   |

**~580,000 instructions/second** (terminal I/O bound)

### 64bits no verbose (10M instructions)

| Run | Wall-clock |
|-----|-----------|
| 1   | 462 ms    |
| 2   | 476 ms    |
| 3   | 472 ms    |
| 4   | 473 ms    |
| 5   | 483 ms    |

**~21,200,000 instructions/second**

### 64bits with verbose -vv (1M instructions)

| Run | Wall-clock |
|-----|-----------|
| 1   | 1.50 s    |
| 2   | 1.50 s    |
| 3   | 1.46 s    |
| 4   | 1.48 s    |
| 5   | 1.42 s    |

**~680,000 instructions/second** (terminal I/O bound)
