#!/usr/bin/env python3
"""
Simple test runner for pymwemu
Run make setup-env before the Python tests.
"""

import os
import sys
import subprocess
import argparse
import time
from pathlib import Path

def run_command(cmd, cwd=None, env=None):
    """Run a command and return success status"""
    print(f"Running: {' '.join(cmd)}")
    try:
        result = subprocess.run(cmd, cwd=cwd, env=env)
        return result.returncode == 0
    except Exception as e:
        print(f"Error running command: {e}")
        return False

def venv_python():
    folder = Path(__file__).resolve().parent / ".env"
    return folder / ("Scripts/python.exe" if os.name == "nt" else "bin/python")

def run_rust_tests(python_exe):
    """Run Rust tests"""
    print("=" * 60)
    print("Running Rust tests...")
    print("=" * 60)
    
    workspace = Path(__file__).resolve().parents[2]
    env = os.environ.copy()
    env.setdefault("PYO3_PYTHON", python_exe)
    if os.name == "nt":
        # The bindings test executable also needs the selected Python DLL at runtime.
        base = subprocess.check_output([env["PYO3_PYTHON"], "-c", "import sys; print(sys.base_prefix)"], text=True).strip()
        env["PATH"] = base + os.pathsep + env.get("PATH", "")
    success = run_command(["cargo", "test", "--locked", "--release", "-p", "libmwemu", "-p", "pymwemu"], cwd=workspace, env=env)
    
    if success:
        print("✓ Rust tests passed!")
    else:
        print("✗ Rust tests failed!")
    
    return success

def run_python_tests(python_exe, test_pattern=None, verbose=False):
    """Run Python tests using the virtual environment"""
    print("=" * 60)
    print("Running Python tests...")
    print("=" * 60)
    
    tests_dir = Path(__file__).parent / "tests"
    
    if test_pattern:
        # Run specific test
        cmd = [python_exe, "-m", "unittest", test_pattern]
    else:
        # Run all tests
        cmd = [python_exe, "-m", "unittest", "discover", "-s", ".", "-p", "test_*.py"]
    
    if verbose:
        cmd.append("-v")
    
    success = run_command(cmd, cwd=tests_dir)
    
    if success:
        print("✓ Python tests passed!")
    else:
        print("✗ Python tests failed!")
    
    return success

def rebuild_extension(python_exe):
    """Rebuild the extension using maturin develop"""
    print("=" * 60)
    print("Rebuilding pymwemu extension...")
    print("=" * 60)
    
    project_root = Path(__file__).parent

    print("Generating stubs...")
    env = dict(os.environ, VIRTUAL_ENV=str(Path(python_exe).parent.parent), PYO3_PYTHON=python_exe)
    if not run_command(["cargo", "run", "--locked", "--bin", "stub_gen"], cwd=project_root, env=env):
        print("✗ Failed to generate stubs!")
        return False
        
    venv_path = project_root / ".env"
    
    if os.name == 'nt':
        maturin_exe = venv_path / "Scripts" / "maturin.exe"
    else:
        maturin_exe = venv_path / "bin" / "maturin"
    
    if not maturin_exe.exists():
        print("Maturin not found in virtual environment!")
        return False
    
    success = run_command([str(maturin_exe), "develop", "--release"], cwd=project_root, env=env)
    
    if success:
        print("✓ Extension rebuilt successfully!")
    else:
        print("✗ Failed to rebuild extension!")
    
    return success

def main():
    parser = argparse.ArgumentParser(description="Run pymwemu tests")
    selection = parser.add_mutually_exclusive_group()
    selection.add_argument("--rust-only", action="store_true", help="Run only Rust tests")
    selection.add_argument("--python-only", action="store_true", help="Run only Python tests")
    parser.add_argument("--test", help="Run specific Python test (e.g., test_basic_functionality)")
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")
    parser.add_argument("--rebuild", action="store_true", help="Rebuild extension before testing")
    
    args = parser.parse_args()
    if args.rust_only and (args.test or args.rebuild):
        parser.error("--test and --rebuild require Python tests")
    
    start_time = time.time()
    
    # Check virtual environment for Python tests
    interpreter = venv_python()
    if not args.rust_only and not interpreter.is_file():
        sys.exit("Missing Python test environment. Run make setup-env in crates/pymwemu.")
    python_exe = str(interpreter) if interpreter.is_file() else sys.executable
    if args.rebuild and not rebuild_extension(python_exe):
        sys.exit(1)
    
    rust_success = True
    python_success = True
    
    # Run tests based on arguments
    if not args.python_only:
        rust_success = run_rust_tests(python_exe)
    
    if not args.rust_only:
        python_success = run_python_tests(python_exe, args.test, args.verbose)
    
    # Summary
    end_time = time.time()
    duration = end_time - start_time
    
    print("=" * 60)
    print("TEST SUMMARY")
    print("=" * 60)
    print(f"Duration: {duration:.2f} seconds")
    
    if not args.python_only:
        print(f"Rust tests: {'PASSED' if rust_success else 'FAILED'}")
    
    if not args.rust_only:
        print(f"Python tests: {'PASSED' if python_success else 'FAILED'}")
    
    overall_success = rust_success and python_success
    print(f"Overall: {'PASSED' if overall_success else 'FAILED'}")
    
    sys.exit(0 if overall_success else 1)

if __name__ == "__main__":
    main()