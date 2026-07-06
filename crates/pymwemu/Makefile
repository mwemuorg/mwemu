# Makefile for pymwemu development and testing

.PHONY: setup-env test test-python test-rust clean clean-env rebuild-test

# Default Python interpreter
PYTHON ?= python3

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
setup-env:
	@echo "Setting up test environment..."
	$(PYTHON) -m venv $(VENV_DIR)
	$(VENV_PYTHON) -m pip install --upgrade pip
	$(VENV_PYTHON) -m pip install maturin
	$(VENV_MATURIN) develop --release
	@echo "Test environment ready!"
	@echo "To activate: source $(VENV_DIR)/bin/activate (Linux/Mac) or $(VENV_DIR)\\Scripts\\activate (Windows)"

# Quick setup using the setup script
setup-quick:
	$(PYTHON) setup_test_env.py

# Run all tests (Rust + Python)
test: test-rust test-python

# Run Rust tests (from parent directory)
test-rust:
	cd .. && cargo test --release

# Run Python tests (requires setup-env first)
test-python:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --python-only

# Run all tests using the test runner
test-all:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py

# Run specific Python test
test-specific:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --test $(TEST) --verbose

# Run Python tests with verbose output
test-verbose:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --python-only --verbose

# Run basic functionality tests only
test-basic:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --test test_basic_functionality --verbose

# Run execution engine tests only
test-execution:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --test test_execution_engine --verbose

# Run binary loading tests only
test-binary:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --test test_binary_loading --verbose

# Run advanced features tests only
test-advanced:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --test test_advanced_features --verbose

# Rebuild extension and test
rebuild-test:
	@if [ ! -f "$(VENV_PYTHON)" ]; then \
		echo "Test environment not found. Run 'make setup-env' first."; \
		exit 1; \
	fi
	$(PYTHON) run_tests.py --rebuild

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
	@echo "  setup-quick     - Quick setup using setup_test_env.py"
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