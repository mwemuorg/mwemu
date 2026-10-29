#!/bin/bash
# Comprehensive Python test runner for pymwemu

set -e  # Exit on any error

echo "Setting up pymwemu test environment..."

# Check if virtual environment exists
VENV_DIR=".env"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    VENV_PYTHON="$VENV_DIR/Scripts/python.exe"
    VENV_ACTIVATE="$VENV_DIR/Scripts/activate"
else
    VENV_PYTHON="$VENV_DIR/bin/python"
    VENV_ACTIVATE="$VENV_DIR/bin/activate"
fi

# Function to handle errors
handle_error() {
    echo "ERROR: $1" >&2
    exit 1
}

# pyo3-stub-gen needs Python >= 3.10 and macOS ships 3.9 as `python3`:
# prefer the newest versioned interpreter on PATH.
PYTHON=""
for p in python3.14 python3.13 python3.12 python3.11 python3.10 python3; do
    if command -v "$p" >/dev/null 2>&1; then
        PYTHON="$p"
        break
    fi
done
[ -n "$PYTHON" ] || handle_error "No python3 interpreter found on PATH"

# Create venv if it doesn't exist
if [ ! -f "$VENV_PYTHON" ]; then
    echo "Creating virtual environment with $PYTHON..."
    "$PYTHON" -m venv "$VENV_DIR" || handle_error "Failed to create virtual environment"
    
    # Activate and install maturin
    source "$VENV_ACTIVATE" || handle_error "Failed to activate virtual environment"
    pip install maturin[patchelf] || handle_error "Failed to install maturin"
else
    echo "Virtual environment found, activating..."
    source "$VENV_ACTIVATE" || handle_error "Failed to activate virtual environment"
fi

# Clean and rebuild pymwemu to ensure fresh build
echo "Cleaning previous builds..."
if [ -d "../target" ]; then
    echo "Removing target directory..."
    rm -rf ../target
fi

if [ -d "deps" ]; then
    echo "Removing deps directory..."
    rm -rf deps
fi

# --- STUB GENERATION ---
echo "Generating Python stubs..."
cargo run --bin stub_gen || handle_error "Failed to generate python stubs"
# -----------------------------------

# Always rebuild to ensure we're testing the latest code
echo "Building pymwemu with maturin..."
maturin develop --release || handle_error "Failed to build pymwemu with maturin develop"

# Verify the module can be imported
echo "Verifying pymwemu installation..."
python -c "import pymwemu; print('✓ pymwemu imported successfully from:', pymwemu.__file__)" || handle_error "Failed to import pymwemu"

# Run simple tests first
echo ""
echo "Running simple tests..."
python -m unittest tests.test_simple -v || handle_error "Simple tests failed"

echo ""
echo "Running all tests..."
python -m unittest discover -v tests/ || handle_error "Full test suite failed"

echo ""
echo "✓ All tests passed successfully!"