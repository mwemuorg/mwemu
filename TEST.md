Do this tests and indicate if passes all the tests, if one test fail dont continue and notify it.
If a step is commented '#' dont do that step

## Prerequisites

If on Apple Silicon (M1/M2/M3/M4), install the x86_64 target first:

```bash
rustup target add x86_64-apple-darwin
```

Fetch the test sample bundle (needed for steps 4, 5, 6):

```bash
make samples
```

Fetch the Windows DLLs the tests load (tests never download them; `make tests` runs this too):

```bash
make symbols
```

## Apple Silicon note

On Apple Silicon hosts, every cargo command must include `--target x86_64-apple-darwin`.
The Makefile handles this automatically if you set:

```bash
export CARGO_TARGET="--target x86_64-apple-darwin"
```

The steps below show the bare commands; add that flag on Apple Silicon or use the Makefile targets.

## Steps

1. `make tests` --> must pass all tests
2. `make test_linux` --> you have to see the result of ls and good termination
3. `make test_windows` --> (takes time) you have to see that arrives at least to 231067159 instructions emulated.
4. `make test_inception` --> arrives to exit: ** syscall exit()  1
5. `make test_syscall` --> (takes a lot of time) you have to see yellow syscalls and you have to see that LdrInitializeThunk is completed: ntdll!LdrInitializeThunk emulated completely.
6. check that format is ok, otherwise fix it: `cargo fmt --all -- --check`
7. check clippy is clean (denies warnings): `make clippy`
   Must exit 0. The pre-existing backlog is grandfathered via crate-level
   `#![allow(...)]` blocks marked "clippy v1 burn-down backlog"; new warnings
   outside that set fail the build. MSRV is 1.95.
