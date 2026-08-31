.PHONY: all tests test-ci clippy clippy-release smoke maps sloppy samples driver

# Extra Cargo target arguments for cross-target checks. On Apple Silicon, use
# CARGO_TARGET="--target x86_64-apple-darwin" as required by AGENTS.md.
CARGO_TARGET ?=

# Keep this list aligned with the workspace default-members and hosted CI.
CI_PACKAGES := --package mwemu --package libmwemu --package rs-header --package cmwemu

# Test samples + Windows DLLs live at the repo root (test/ and maps/), the single
# canonical location shared by the CLI and the tests (tests/helpers.rs resolves
# them relative to the workspace root). The bundle is non-redistributable, so
# it's gitignored and fetched on demand.
TEST_DIR := test

all:
	cargo build --locked --release $(CARGO_TARGET)

# Full local run: fetch the sample bundle, then run the default CI packages.
tests: samples
	cargo build --locked $(CI_PACKAGES) $(CARGO_TARGET)
	cargo test --locked --verbose $(CI_PACKAGES) $(CARGO_TARGET)

# CI run: no bundle, no network. Binary-dependent tests skip themselves via the
# `sample!` macro when their sample isn't present, so CI runs the self-contained
# suite green. (The committed maps support files — banzai.csv, loader.exe — are
# enough for the tests that only need a maps folder.)
test-ci:
	cargo build --locked $(CI_PACKAGES) $(CARGO_TARGET)
	cargo test --locked --verbose $(CI_PACKAGES) $(CARGO_TARGET)

# Lint the same packages and targets as hosted CI. Existing warnings are reported
# but are not denied until the current warning backlog is addressed separately.
clippy:
	cargo clippy --locked $(CI_PACKAGES) --all-targets $(CARGO_TARGET)

clippy-release:
	cargo clippy --locked --release $(CI_PACKAGES) --all-targets $(CARGO_TARGET)

# Kernel-mode test target: builds drivers/linux/tlm into test/linux_uaf_driver.ko,
# the deliberately vulnerable .ko the kernel emulation tests load. Needs the
# running kernel's headers (/lib/modules/$(uname -r)/build); the tests skip
# themselves when the artefact is absent, so this is optional.
driver:
	$(MAKE) -C drivers/linux/tlm install TESTDIR=$(abspath $(TEST_DIR))

# Sample PE bundle (msgbox, enigma, ...), fetched once from the mwemu release
# assets. Everything that needs a sample depends on this, so a fresh checkout
# self-heals instead of running against a missing file (which the loader would
# otherwise misdetect as shellcode and crash). The file target is the extracted
# marker; `samples` is a friendly phony alias.
$(TEST_DIR)/exe64win_msgbox.bin:
	@echo "[samples] fetching test bundle into $(TEST_DIR)/ from mwemuorg/mwemu releases ..."
	@if which wget >/dev/null 2>&1; then \
		wget -q -O test.zip https://github.com/mwemuorg/mwemu/releases/download/maps/test.zip; \
	else \
		curl -fsSL -o test.zip https://github.com/mwemuorg/mwemu/releases/download/maps/test.zip; \
	fi
	@unzip -o -P mwemuTestSystem test.zip; rm -f test.zip

samples: $(TEST_DIR)/exe64win_msgbox.bin

# Optional end-to-end smoke test: emulate a sample PE (needs network for the
# winver DLL fetch).
smoke: all samples
	./target/release/mwemu -f $(TEST_DIR)/exe64win_msgbox.bin -6 --winver win11 -e 200000

# Pre-fetch the genuine x64 Windows system DLLs from Microsoft's symbol server
# into maps/windows/x86_64/. Optional warm-up: libmwemu also auto-fetches any
# missing DLL on demand. Idempotent: skips if the maps are already present.
maps: all samples
	@mkdir -p maps/windows/x86_64
	@if [ -f maps/windows/x86_64/kernelbase.dll ]; then \
		echo "[maps] x64 system DLLs already present"; \
	else \
		echo "[maps] pre-fetching x64 system DLLs from the symbol server..."; \
		./target/release/mwemu -f $(TEST_DIR)/exe64win_msgbox.bin -6 --maps maps/windows/x86_64/ -e 300000 --banzai >/dev/null 2>&1 || true; \
		echo "[maps] warmed $$(ls maps/windows/x86_64/*.dll 2>/dev/null | wc -l) DLLs into maps/windows/x86_64/ (more are auto-fetched on demand)"; \
	fi

sloppy:
	-python3 scripts/sloppy.py

# Handy manual runs
test_syscall: samples
	cargo run --release -- -f $(TEST_DIR)/exe64win_msgbox.bin -6 --syscall-mode --winver win11
test_linux:
	cargo run --release -- -f /bin/ls -A '"-l"' -6
test_windows: samples
	cargo run --release -- -f $(TEST_DIR)/exe64win_enigma.bin -6 --winver win11  -v
test_inception:
	cargo run --release -- -f target/release/mwemu -6 -v
test_enigma:
	cargo run --release -- -f  test/exe64win_enigma.bin -6 -v
test_tls:
	cargo run --release -- -f  test/exe64win_mingw.bin -6 -v

