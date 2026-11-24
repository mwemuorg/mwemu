# Testing mwemu

## Test types

Each command runs the suite that includes the test type. See [Running the suites](#running-the-suites) for fixture preparation and group filters.

| Test type | Where the input comes from | Cargo command |
| --- | --- | --- |
| Unit test with data built in Rust | The test constructs CPU state, instructions, API arguments or a synthetic image. | `cargo test` |
| Unit or parser test with a checked-in binary | `include_bytes!` embeds a fixture such as `loader32.exe` or `loader64.exe` in the `rs-header` tests. | `cargo test` |
| Self-contained guest program | Loader tests emulate checked-in binaries from `crates/libmwemu/src/tests/fixtures/`. | `cargo test` |
| Guest program with prepared dependencies | The Windows x86/x64 hello-world EXEs are checked in. Their loader tests read DLLs from the prepared maps directory. | `cargo test -p libmwemu --features asset-tests` |
| Shellcode with inline bytes | The Linux x86 `int 0x80` test defines its instructions in Rust. | `cargo test` |
| Shellcode from the release archive | `sample!(...)` locates the raw binary in `test/`. The arithmetic-loop test uses this path. | `cargo test -p libmwemu --features asset-tests` |
| Guest program from the release archive | `test/` holds PE and ELF programs, packed executables and the bundled Linux kernel module. Windows SSDT tests also execute the loader in a Windows system image. | `cargo test -p libmwemu --features asset-tests` |
| Guest compiled by the test | The host Go compiler builds a checked-in source file in a temporary directory. The test emulates the resulting Linux amd64 ELF. | `cargo test -p libmwemu --features host-tests` |
| Guest with host-prepared artifacts | Windows and macOS driver tests load locally built drivers. Mach-O fork tests use dylibs extracted from macOS. | `cargo test -p libmwemu --features host-tests` |
| Additional Linux driver checks | These cases use the same `.ko` as the regular fixture suite and exercise the kernel memory diagnostics. | `cargo test -p libmwemu --features asset-tests,host-tests` |
| Manual corpus or host check | The `rs-header` bulk parser uses an external corpus. Other manual tests use a host executable or system libraries. | `cargo test -- --ignored` |
| Manual Winver shellcode check | The test uses a release sample and provisions its own Windows DLL cache. | `cargo test -p libmwemu --features asset-tests sc64win_winver -- --ignored` |

The loader modules group ELF, Mach-O and PE tests by guest format. Cargo features select tests by what they need to run. `asset-tests` adds tests that use prepared files. `host-tests` adds tests that use host tools or locally prepared artifacts. An attribute such as `#[cfg(feature = "host-tests")]` on a module applies to every test in it. A test gated by both features needs both. When a test requires a specific host OS or architecture, its source uses `#[cfg(target_os = ...)]` or `#[cfg(target_arch = ...)]`.

`#[ignore = "..."]` keeps a test in the compiled suite and skips it on an ordinary run. Append `-- --list` to list compiled tests or `-- --ignored --list` to list ignored tests. To run one ignored test, pass its name filter before `-- --ignored`.

Flag timings and the packed PE startup benchmark use `cargo bench`. Differential tests use their own harnesses. See [Manual checks and other test tools](#manual-checks-and-other-test-tools).

## Running the suites

Run these commands from the workspace root. When checking a change by hand, stop at the first failed check and report the command and its output.

| Goal | Command |
| --- | --- |
| Run the offline suite for the default workspace packages | `cargo test --locked` |
| Run the full release suite with prepared assets | `python scripts/sync_assets.py`, then `cargo test-assets` |
| Run the shellcode group with prepared inputs | `cargo test-assets -p libmwemu --lib tests::shellcode::` |
| Run ELF guests | `python scripts/sync_assets.py test-fixtures`, then `cargo test-elf` |
| Run checked-in Mach-O guests | `cargo test-macho` |
| Include fork and pthread guests on macOS | `make dyld`, then `cargo test-macho --features host-tests` |
| Run the Windows driver lifecycle | `make windows-driver`, then `cargo test-windows-driver` |
| Run PE guests, including x64 SSDT startup | `python scripts/sync_assets.py`, then `cargo test-pe` |

The [Cargo aliases](.cargo/config.toml) run on the selected host. For example, `cargo test-macho` works on Windows. The Mach-O fork tests in the same module also need `host-tests` and dylibs extracted from macOS.

The `asset-tests` feature also works in debug builds. Use release for long guest runs and throughput checks. For a focused run, pass a module filter such as `tests::loaders::pe::packed::` or `tests::loaders::pe::tls_callbacks_tests::` to the fixture suite. The packed PE group checks startup and GUI behavior. The x64 SSDT group checks strict-mode entry for MessageBox, MinGW and Enigma, a return from `MessageBoxA`, Enigma execution to 50 million instructions and MessageBox loader startup in success simulation.

The x64 SSDT tests execute the Windows loader in `ntdll.dll` and intercept its system calls. The packed PE tests use Rust WinAPI handlers. On x64 Windows both groups read DLLs from the host's `System32`. They can run after `python scripts/sync_assets.py test-fixtures` with no DLLs in the checkout. The tests disable symbol downloads.

To use a different Windows image, set `MWEMU_TEST_SYSTEM32` to its x64 system directory. On other hosts these groups use `maps/windows/x86_64` by default. For build `26100.7920`, run `python scripts/sync_assets.py --dll gdi32full.dll --dll wintypes.dll` to include the two libraries loaded by name during GUI startup. The SSDT tests also need the Windows NLS data described below.

### Apple Silicon

With a native Rust toolchain on Apple Silicon, run `cargo test --locked`. The macOS CI job selects the same target explicitly with `cargo test --locked --target aarch64-apple-darwin`.

To compare a native result with an Intel build on the same Mac, use Rosetta to run the x86_64 test binary. For example, this can help isolate differences in floating-point results between hosts.

```sh
rustup target add x86_64-apple-darwin
cargo test --locked --target x86_64-apple-darwin
```

Pass the target directly to an alias, for example `cargo test-macho --target x86_64-apple-darwin`. With Make, use `CARGO_TARGET="--target x86_64-apple-darwin"`.

### Syncing assets

Run `python scripts/sync_assets.py` to prepare both groups. The `asset-tests` feature adds tests that need prepared files to the default suite. A selected test fails if a required file is missing.

| Command | Files |
| --- | --- |
| `python scripts/sync_assets.py test-fixtures` | Restores missing or empty files listed in the release archive. It downloads the archive from GitHub on first use and reuses `test/.samples.zip` on later runs. |
| `python scripts/sync_assets.py maps` | Prepares the core x86 and x64 Windows DLLs, then follows ordinary PE imports from the available fixtures and their system libraries. Use `--dll NAME` for a DLL loaded by name at runtime. |
| `python scripts/sync_assets.py --offline` | Uses local inputs and reports missing assets. Without a cached fixture archive, it reports that it cannot check archive membership. Tests can still use extracted files. |

The installer uses the Python 3.9+ standard library. Use `--test-dir PATH` or `make sync-test-fixtures TEST_DIR=PATH` to restore fixtures elsewhere. Make passes `TEST_DIR` to both the installer and the test process. For a direct Cargo run, set `MWEMU_TEST_DIR` to use the same directory. Relative paths are resolved against the workspace root. To fetch an updated release archive, remove the cached `.samples.zip` and run the installer again. It preserves existing nonempty fixtures.

On x64 Windows, `make sync-maps` copies missing DLLs from the host. On other hosts it keeps usable maps and resolves missing DLLs through Winbindex and Microsoft's symbol server. `make sync-maps-symsrv` selects that source explicitly on every host. For symbol-server DLLs, the installer checks the SHA-256 against the index entry for the selected architecture and Windows build. Use `--build BUILD.REVISION` to select a build. If the exact component revision is unavailable, the installer uses the latest indexed revision at or below it in the same build family. Windows NLS tables come from the host or a local Windows image cache.

`make tests`, `make test_linux` and `make test_windows` combine the preparation and test commands shown above. On hosts without an x64 Windows system directory, prepare the GUI dependencies described above first. `make test-ci` and `scripts/test_coverage.sh` use the default suite. `scripts/test-pr.sh` includes `asset-tests` when the checked-out PR declares it, in release mode unless the caller explicitly selects packages or features. An explicit Cargo profile takes precedence.

### Host prerequisites

Use `cargo test --locked -p libmwemu --lib --features host-tests` followed by the group filter. For the Linux driver group, use `--features asset-tests,host-tests` to include its additional cases.

| Group filter | Preparation |
| --- | --- |
| `tests::loaders::elf::from_source::` | Put Go on PATH. |
| `tests::kernel::linux_uaf_tests::` | The `asset-tests` suite uses the bundled `.ko` for loading and ordinary I/O. Run `make linux-driver` to rebuild the module against the running kernel’s headers. |
| `tests::kernel::windows_driver_tests::` | Run `make windows-driver` with the MinGW-w64 compiler and headers. |
| `tests::kernel::macos_uaf_tests::` | Run `make macos-driver` with the macOS SDK. |
| `tests::loaders::macho::macho64_fork::` | Run `make dyld` on macOS to extract the system libraries. |

## Manual checks and other test tools

- The ignored Windows ARM64 test needs ARM64 DLLs. The ignored Winver shellcode test provisions its own symbol cache. Linux x86-64 also has explicit checks for the host’s `/bin/ls` and the built emulator.
- Set `RS_HEADER_CORPUS_DIR` to an external PE/ELF corpus before running `cargo test --locked -p rs-header --test bulk_parse -- --ignored`.
- Run `cargo bench -p libmwemu --bench lazy_flags` for flag timings. With prepared assets, `cargo bench-pe` measures a 20-million-instruction packed PE run. The [x86 instruction harness](crates/mwemu-x86-test/README.md) compares instruction results with a reference. Standalone fuzz targets live under `fuzz/`.
- For the optional Rosetta check on Apple Silicon, rebuild the Python bindings after changing `libmwemu` (`maturin develop --release` in `crates/pymwemu` with its virtual environment active). From the workspace root, run `scripts/rosetta-difftest/run.sh`. Both sweeps must finish. Review their mismatch reports even if the script exits with status 0. The [Rosetta guide](scripts/rosetta-difftest/README.md) lists the known differences for `cmpxchg`, `adcx`/`adox` and 16-bit `shld`/`shrd` with counts above 16.
- See the Python bindings docs for [build prerequisites](crates/pymwemu/README.md#build-from-source) and [tests](crates/pymwemu/TESTING.md).

On x86-64 Linux with glibc, run `cargo test --release --locked -p mwemu --test self_emulation -- --ignored`. Cargo builds the CLI and passes its path to the test. The emulator loads the CLI as an ELF guest with the host’s dynamic loader and libc. The test passes once the guest has executed at least 900,000 instructions, regardless of how the run ends.

For a manual directory-listing trace on Linux, use `cargo run --release -p mwemu -- -f /bin/ls -6 --args=-l`. On Apple Silicon, use `--aarch64` in place of `-6`. For a MessageBox syscall trace, prepare the release fixture and run `cargo run --release -p mwemu -- -f test/exe64win_msgbox.bin -6 --syscall-mode --winver win11`.

## CI

Linux, macOS and Windows run the offline Rust suite and the asset installer tests (`python scripts/test_sync_assets.py`). The offline suite checks Linux heap diagnostics directly against the emulated allocator.

| Runner | Additional coverage |
| --- | --- |
| Linux | Runs the archived ELF fixtures and builds the Go guest from source. |
| macOS (Apple Silicon) | Extracts system dylibs with `make dyld` and runs the Mach-O suite, including fork and pthread guests. |
| Windows (x64) | Runs the release asset suite. The SSDT and packed PE tests use the runner’s system DLLs. MinGW builds a driver fixture for emulated entry, I/O, routine lookup, error recovery and unload. |

Use `cargo fmt --all -- --check` to check formatting and `make fmt` to apply it. CI and `make clippy` use `cargo clippy-all -- -D warnings`. `make msrv` checks the minimum supported Rust version (1.95). Linux CI also checks API, syscall and kernel module coverage (locally, run `make api-modules`).
