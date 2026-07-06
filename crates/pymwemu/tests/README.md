# PyMwemu Test Suite

This directory contains comprehensive tests for the pymwemu Python bindings, inspired by the libmwemu Rust test suite.

## Test Structure

- `test_basic_functionality.py` - Basic emulator operations (registers, memory, initialization)
- `test_execution_engine.py` - Instruction execution, breakpoints, hooks
- `test_binary_loading.py` - Binary loading and shellcode execution
- `test_advanced_features.py` - Advanced features (FPU, serialization, tracing)
- `test_runner.py` - Test runner utility
- `conftest.py` - Test configuration and fixtures

## Running Tests

### Method 1: Using the Makefile (Recommended)

```bash
# First-time setup
make setup-env       # Creates .env venv and builds extension

# Run all tests (Rust + Python)
make test

# Run only Python tests
make test-python

# Run specific test categories
make test-basic      # Basic functionality
make test-execution  # Execution engine
make test-binary     # Binary loading
make test-advanced   # Advanced features

# Run with verbose output
make test-verbose

# Run specific test by name
make test-specific TEST=test_basic_functionality
```

### Method 2: Manual setup (your preferred method)

```bash
# Create virtual environment
python -m venv .env

# Activate it
source .env/bin/activate  # Linux/Mac
# or
.env\Scripts\activate     # Windows

# Install maturin and build extension
pip install maturin
maturin develop --release

# Now you can run tests directly
python -m unittest discover -v tests/
```

### Method 3: Using the Python test runner

```bash
# After setup, use the test runner
python run_tests.py                    # All tests
python run_tests.py --python-only      # Python only
python run_tests.py --rust-only        # Rust only
python run_tests.py --test test_basic_functionality --verbose
python run_tests.py --rebuild          # Rebuild extension first
```

### Method 4: Quick setup script

```bash
# One-command setup
python setup_test_env.py

# Then run tests
python run_tests.py
```

## Test Categories

### Basic Functionality Tests
- Emulator initialization (32-bit and 64-bit)
- Register operations (get/set, subregisters)
- Memory operations (read/write byte/word/dword/qword)
- Memory boundary conditions with banzai mode
- String operations (ASCII and wide strings)
- Stack operations (push/pop)
- Memory search functionality
- Error condition handling

### Execution Engine Tests
- Basic instruction execution
- Arithmetic operations
- Memory access instructions
- Stack instructions
- Breakpoint functionality
- Instruction counting
- Flag operations
- 64-bit execution
- Execution limits and timeouts
- Hook functionality (if available)

### Binary Loading Tests
- 32-bit shellcode execution
- 64-bit shellcode execution
- String manipulation shellcode
- API emulation basics
- PEB/TEB structure emulation
- Complex shellcode patterns (loops, conditionals)
- Memory scanning shellcode
- Encryption/decryption shellcode
- Shellcode with API calls

### Advanced Features Tests
- FPU operations (if available)
- State serialization/deserialization
- Memory protection
- Complex memory operations
- Advanced memory search
- Instruction tracing
- Register tracing
- Advanced memory mapping
- Console and output settings
- Error handling edge cases
- Performance scenarios
- 64-bit specific features

## Requirements

- Python 3.7+
- maturin (for building the extension)
- Rust toolchain
- pymwemu dependencies

## Notes

- Tests automatically enable banzai mode for safer error handling
- Some advanced features may not be available in Python bindings
- Tests that require unavailable APIs are automatically skipped
- All tests are designed to be independent and can run in any order

## Troubleshooting

If tests fail to run:

1. Ensure maturin is installed: `pip install maturin`
2. Build the extension: `maturin develop --release`
3. Check that pymwemu is properly installed
4. Verify Rust toolchain is available for cargo tests

For specific test failures, run with verbose output to see detailed error messages.