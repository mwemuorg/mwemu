.PHONY: all tests clippy clippy-release smoke maps sloppy samples

# Build natively for the host — no cross-compile. The emulator is pure Rust and
# host-arch agnostic (it emulates x86/x64/arm64 in software), so a Mac builds for
# the Mac, Linux for Linux and Windows for Windows.
CARGO_TARGET :=

# Test samples live where libmwemu's tests look for them: CARGO_MANIFEST_DIR/test
# = crates/libmwemu/test/ (see tests/helpers.rs::test_data_path). The bundle is
# non-redistributable, so it's gitignored and fetched on demand.
TEST_DIR := crates/libmwemu/test

all:
	cargo build --release $(CARGO_TARGET)

# Full workspace test suite. Depends on `samples` because many loader/shellcode
# tests emulate real binaries from the sample bundle (in $(TEST_DIR)/).
tests: samples
	cargo build $(CARGO_TARGET)
	cargo test --verbose $(CARGO_TARGET)

# Lint. `clippy-release` is what CI runs; mirrors the libmwemu setup.
clippy:
	cargo clippy --workspace

clippy-release:
	cargo clippy --release --lib --bins $(CARGO_TARGET)

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
	@unzip -o -P mwemuTestSystem test.zip -d crates/libmwemu; rm -f test.zip

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
	cargo run --release -- -f $(TEST_DIR)/exe64win_enigma.bin -6 --winver win11 -v
test_inception:
	cargo run --release -- -f target/release/mwemu -6 -v
