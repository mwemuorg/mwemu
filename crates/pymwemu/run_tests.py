#!/usr/bin/env python3
"""
Simple test runner for pymwemu
Assumes you have already set up the environment with setup_test_env.py
"""

import os
import sys
import subprocess
import argparse
import time
from pathlib import Path

def run_command(cmd, cwd=None):
    """Run a command and return success status"""
    print(f"Running: {' '.join(cmd)}")
    try:
        result = subprocess.run(cmd, cwd=cwd)
        return result.returncode == 0
    except Exception as e:
        print(f"Error running command: {e}")
        return False

def check_venv():
    """Check if we're in the test virtual environment"""
    project_root = Path(__file__).parent
    venv_path = project_root / ".env"
    
    if os.name == 'nt':
        python_exe = venv_path / "Scripts" / "python.exe"
    else:
        python_exe = venv_path / "bin" / "python"
    
    if not python_exe.exists():
        print("Test environment not found!")
        print("Please run: python setup_test_env.py")
        return False, None
    
    return True, str(python_exe)

def run_rust_tests():
    """Run Rust tests"""
    print("=" * 60)
    print("Running Rust tests...")
    print("=" * 60)
    
    # Change to parent directory to run cargo tests
    parent_dir = Path(__file__).parent.parent
    success = run_command(["cargo", "test", "--release"], cwd=parent_dir)
    
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
    if not run_command(["cargo", "run", "--bin", "stub_gen"], cwd=project_root):
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
    
    success = run_command([str(maturin_exe), "develop", "--release"], cwd=project_root)
    
    if success:
        print("✓ Extension rebuilt successfully!")
    else:
        print("✗ Failed to rebuild extension!")
    
    return success

def main():
    parser = argparse.ArgumentParser(description="Run pymwemu tests")
    parser.add_argument("--rust-only", action="store_true", help="Run only Rust tests")
    parser.add_argument("--python-only", action="store_true", help="Run only Python tests")
    parser.add_argument("--test", help="Run specific Python test (e.g., test_basic_functionality)")
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")
    parser.add_argument("--rebuild", action="store_true", help="Rebuild extension before testing")
    
    args = parser.parse_args()
    
    start_time = time.time()
    
    # Check virtual environment for Python tests
    if not args.rust_only:
        venv_ok, python_exe = check_venv()
        if not venv_ok:
            sys.exit(1)
        
        # Rebuild extension if requested
        if args.rebuild:
            if not rebuild_extension(python_exe):
                sys.exit(1)
    
    rust_success = True
    python_success = True
    
    # Run tests based on arguments
    if not args.python_only:
        rust_success = run_rust_tests()
    
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