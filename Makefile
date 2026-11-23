.PHONY: all tests test-ci clippy clippy-release fmt msrv api-modules complexity complexity-gate sloppy
.PHONY: sync-assets sync-test-fixtures sync-maps sync-maps-symsrv linux-driver macos-driver windows-driver dyld smoke
.PHONY: test_linux test_macos test_windows test_sys test_speed test_fuzz

# Optional Cargo arguments for cross-target checks. See TEST.md for the Apple Silicon example.
CARGO_TARGET ?=

# The installer and Cargo resolve relative fixture paths from the workspace root.
TEST_DIR ?= test
export MWEMU_TEST_DIR := $(TEST_DIR)
ifeq ($(OS),Windows_NT)
PYTHON ?= python
else
PYTHON ?= python3
endif
SYNC_ASSETS = $(PYTHON) scripts/sync_assets.py

all:
	cargo build --locked --release $(CARGO_TARGET)

tests: sync-assets
	cargo test-assets $(CARGO_TARGET)

test-ci:
	cargo test --locked $(CARGO_TARGET)

test_linux: sync-test-fixtures
	cargo test-elf $(CARGO_TARGET)

test_macos:
	cargo test-macho $(CARGO_TARGET)

test_windows: sync-assets
	cargo test-pe $(CARGO_TARGET)

test_speed: sync-assets
	cargo bench-pe $(CARGO_TARGET)

fmt:
	cargo fmt --all

msrv:
	cargo +1.95 check --locked $(CARGO_TARGET)

clippy-release: profile = --release
clippy clippy-release:
	cargo clippy-all $(profile) $(CARGO_TARGET) -- -D warnings

api-modules:
	$(PYTHON) scripts/check_api_modules.py --self-test
	$(PYTHON) scripts/check_api_modules.py $(CARGO_TARGET)

# Rebuild the Linux fixture with the running kernel headers.
linux-driver:
	$(MAKE) -C drivers/linux/tlm install TESTDIR=$(abspath $(TEST_DIR))

# Windows kernel-mode test target: builds drivers/windows/tlm into
# test/windows_uaf_driver.sys with mingw-w64 (x86_64-w64-mingw32-gcc and its
# DDK headers).
windows-driver:
	$(MAKE) -C drivers/windows/tlm install TESTDIR=$(abspath $(TEST_DIR))

# macOS kext test target: builds drivers/macos/tlm into
# test/macos_uaf_driver.kext (arm64e). Needs the macOS SDK + clang; the
# kext tests use the resulting bundle.
macos-driver:
	$(MAKE) -C drivers/macos/tlm install TESTDIR=$(abspath $(TEST_DIR))

sync-assets:
	$(SYNC_ASSETS)

sync-test-fixtures:
	$(SYNC_ASSETS) test-fixtures

sync-maps:
	$(SYNC_ASSETS) maps

sync-maps-symsrv:
	$(SYNC_ASSETS) maps --source symbol-server

# Optional end-to-end smoke test: emulate a sample PE (needs network for the
# winver DLL fetch).
smoke: sync-test-fixtures
	cargo run --locked --release -p mwemu $(CARGO_TARGET) -- -f "$(TEST_DIR)/exe64win_msgbox.bin" -6 --winver win11 -e 200000

sloppy:
	-$(PYTHON) scripts/sloppy.py

# Extract macOS dylibs from the local dyld shared cache into maps/macos/aarch64/.
# Requires macOS with dsc_extractor.bundle (ships with the OS).  The loader
# strips directory prefixes, so every dylib lands flat in the target folder.
DYLD_MAPS := maps/macos/aarch64
DYLD_EXTRACTOR := scripts/extract_dyld_cache

$(DYLD_EXTRACTOR): scripts/extract_dyld_cache.c
	$(CC) -o $@ $<

dyld: $(DYLD_EXTRACTOR)
	@echo "[dyld] extracting dyld shared cache (this takes a minute)..."
	@set -eu; scratch=$$(mktemp -d); \
	trap 'rm -rf "$$scratch"' EXIT; \
	./$(DYLD_EXTRACTOR) "$$scratch"; \
	mkdir -p "$(DYLD_MAPS)"; \
	for f in "$$scratch"/usr/lib/*.dylib "$$scratch"/usr/lib/system/*.dylib; do \
		[ -f "$$f" ] || continue; \
		NAME=$$(basename "$$f"); \
		if git ls-files --error-unmatch "$(DYLD_MAPS)/$$NAME" >/dev/null 2>&1; then \
			echo "  skip $$NAME (tracked stub)"; \
		else \
			cp "$$f" "$(DYLD_MAPS)/$$NAME"; \
		fi; \
	done; \
	echo "[dyld] $$(ls $(DYLD_MAPS)/*.dylib 2>/dev/null | wc -l | tr -d ' ') dylibs in $(DYLD_MAPS)/"

# Manual CLI driver check.
test_sys: windows-driver
	cargo run --release -- -f drivers/windows/tlm/tlm.sys -6 --ioctl 0x80002010:304d4c540010000000100000000000000000000000000000:24 -vv

# Fuzz smoke test (30s each by default, nightly + cargo-fuzz required).
# Uses x86.dict for smarter input generation. Panics are caught (not crashes),
# so the fuzzer only stops on real memory-safety bugs.
FUZZ_TIME ?= 30
FUZZ_DICT := -dict=x86.dict
test_fuzz:
	cd fuzz && ASAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run fuzz_load_code -- -max_total_time=$(FUZZ_TIME) $(FUZZ_DICT)
	cd fuzz && ASAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run fuzz_run_shellcode -- -max_total_time=$(FUZZ_TIME) $(FUZZ_DICT)
	cd fuzz && ASAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run fuzz_load_code64 -- -max_total_time=$(FUZZ_TIME) $(FUZZ_DICT)
	cd fuzz && ASAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run fuzz_run_shellcode64 -- -max_total_time=$(FUZZ_TIME) $(FUZZ_DICT)
	@echo "[fuzz] all 4 targets passed ($(FUZZ_TIME)s each)"

complexity:
	-@$(PYTHON) scripts/complexity.py --top 50

complexity-gate:
	@bash scripts/complexity-gate.sh
