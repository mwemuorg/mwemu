# Makefile for pymwemu development and testing

.PHONY: setup-env setup-quick test test-all test-python test-rust test-specific test-verbose
.PHONY: test-basic test-execution test-binary test-advanced rebuild-test clean clean-env clean-all activate-help help

# Default Python interpreter. pyo3-stub-gen needs Python >= 3.10 and macOS
# ships 3.9 as `python3`, so prefer the newest versioned interpreter on PATH
# (override with `make PYTHON=/path/to/python3.x ...`).
ifeq ($(OS),Windows_NT)
PYTHON ?= python
else
PYTHON ?= $(shell for p in python3.14 python3.13 python3.12 python3.11 python3.10 python3; do command -v $$p >/dev/null 2>&1 && { echo $$p; break; }; done)
endif

# Virtual environment paths
VENV_DIR = .env
ifeq ($(OS),Windows_NT)
    VENV_PYTHON = $(VENV_DIR)/Scripts/python.exe
    VENV_MATURIN = $(VENV_DIR)/Scripts/maturin.exe
else
    VENV_PYTHON = $(VENV_DIR)/bin/python
    VENV_MATURIN = $(VENV_DIR)/bin/maturin
endif

# Setup test environment (your preferred method)
setup-env: export VIRTUAL_ENV := $(abspath $(VENV_DIR))
setup-env:
	@echo "Setting up test environment..."
	$(PYTHON) -m venv $(VENV_DIR)
	"$(VENV_PYTHON)" -m pip install --upgrade pip
	"$(VENV_PYTHON)" -m pip install maturin
	"$(VENV_MATURIN)" develop --release
	@echo "Test environment ready!"
	@echo "To activate: source $(VENV_DIR)/bin/activate (Linux/Mac) or $(VENV_DIR)\\Scripts\\activate (Windows)"

setup-quick: setup-env

RUN_TESTS = $(PYTHON) run_tests.py

test test-all:
	$(RUN_TESTS)

test-rust:
	$(RUN_TESTS) --rust-only

test-python:
	$(RUN_TESTS) --python-only

test-verbose:
	$(RUN_TESTS) --python-only --verbose

test-basic: TEST = test_basic_functionality
test-execution: TEST = test_execution_engine
test-binary: TEST = test_binary_loading
test-advanced: TEST = test_advanced_features
test-basic test-execution test-binary test-advanced test-specific:
	$(RUN_TESTS) --python-only --verbose --test "$(if $(strip $(TEST)),$(TEST),$(error specify TEST for $@))"

rebuild-test:
	$(RUN_TESTS) --rebuild

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/
	rm -rf tests/__pycache__/
	rm -rf tests/.pytest_cache/
	find . -name "*.pyc" -delete
	find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true

# Clean virtual environment
clean-env:
	rm -rf $(VENV_DIR)

# Full clean
clean-all: clean clean-env

# Manual activation instructions
activate-help:
	@echo "To manually activate the test environment:"
ifeq ($(OS),Windows_NT)
	@echo "  $(VENV_DIR)\\Scripts\\activate"
else
	@echo "  source $(VENV_DIR)/bin/activate"
endif
	@echo ""
	@echo "Then you can run tests directly:"
	@echo "  python -m unittest discover -v tests/"

# Help
help:
	@echo "Available targets:"
	@echo "  setup-env       - Set up virtual environment and build extension"
	@echo "  setup-quick     - Alias for setup-env"
	@echo "  test            - Run all tests (Rust + Python)"
	@echo "  test-rust       - Run Rust tests only"
	@echo "  test-python     - Run Python tests only"
	@echo "  test-all        - Run all tests using test runner"
	@echo "  test-verbose    - Run Python tests with verbose output"
	@echo "  test-basic      - Run basic functionality tests"
	@echo "  test-execution  - Run execution engine tests"
	@echo "  test-binary     - Run binary loading tests"
	@echo "  test-advanced   - Run advanced features tests"
	@echo "  test-specific   - Run specific test (use TEST=test_name)"
	@echo "  rebuild-test    - Rebuild extension and run tests"
	@echo "  clean           - Clean build artifacts"
	@echo "  clean-env       - Clean virtual environment"
	@echo "  clean-all       - Clean everything"
	@echo "  activate-help   - Show manual activation instructions"
	@echo "  help            - Show this help message"
	@echo ""
	@echo "Example usage:"
	@echo "  make setup-env"
	@echo "  make test"
	@echo "  make test-specific TEST=test_basic_functionality"