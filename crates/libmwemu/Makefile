# libmwemu — build & test helpers (standalone repo).

CARGO_TARGET :=
ifeq ($(shell uname),Darwin)
	CARGO_TARGET := --target x86_64-apple-darwin
endif

# Password-protected bundle with the sample binaries (test/) and the
# proprietary Windows DLLs (maps/) that can't be redistributed in-repo.
# Published as a release asset on mwemuorg/libmwemu (tag: tests).
TEST_ZIP_URL := https://github.com/mwemuorg/libmwemu/releases/download/tests/test.zip
TEST_ZIP_PASSWORD := mwemuTestSystem

.PHONY: build release tests fmt clippy clippy-release clean

build:
	cargo build $(CARGO_TARGET)

release:
	cargo build --release $(CARGO_TARGET)

# Fetch the test bundle only when the extracted data is missing, drop the zip,
# then run the suite.
tests:
	@if [ -d test ] && [ -d maps ]; then \
		echo "test bundle already present; skipping download"; \
	else \
		echo "fetching test bundle..."; \
		if which wget >/dev/null 2>&1; then \
			wget -q -O test.zip $(TEST_ZIP_URL); \
		else \
			curl -fsSL -o test.zip $(TEST_ZIP_URL); \
		fi; \
		unzip -o -P $(TEST_ZIP_PASSWORD) test.zip; \
		rm -f test.zip; \
	fi
	cargo test --release --verbose $(CARGO_TARGET)

fmt:
	cargo fmt --check

clippy:
	cargo clippy

clippy-release:
	cargo clippy --release --lib --bins $(CARGO_TARGET)

clean:
	cargo clean
	rm -rf test maps test.zip
