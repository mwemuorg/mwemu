#!/usr/bin/env python3
"""
Test configuration and fixtures for pymwemu tests
"""

import sys
import os
import tempfile
import shutil

# Add pymwemu to Python path
current_dir = os.path.dirname(os.path.abspath(__file__))
pymwemu_dir = os.path.dirname(current_dir)
sys.path.insert(0, pymwemu_dir)

# Test configuration
TEST_CONFIG = {
    'verbose': False,
    'temp_dir': None,
    'cleanup': True
}

def setup_test_environment():
    """Set up test environment"""
    # Create temporary directory for test files
    TEST_CONFIG['temp_dir'] = tempfile.mkdtemp(prefix='pymwemu_test_')
    
    # Set environment variables if needed
    os.environ['PYMWEMU_TEST_MODE'] = '1'
    
    return TEST_CONFIG['temp_dir']

def cleanup_test_environment():
    """Clean up test environment"""
    if TEST_CONFIG['cleanup'] and TEST_CONFIG['temp_dir']:
        shutil.rmtree(TEST_CONFIG['temp_dir'], ignore_errors=True)
    
    # Clean up environment variables
    if 'PYMWEMU_TEST_MODE' in os.environ:
        del os.environ['PYMWEMU_TEST_MODE']

def get_test_data_path():
    """Get path to test data directory"""
    return os.path.join(current_dir, 'data')

def create_test_binary(filename, content):
    """Create a test binary file"""
    if not TEST_CONFIG['temp_dir']:
        setup_test_environment()
    
    filepath = os.path.join(TEST_CONFIG['temp_dir'], filename)
    with open(filepath, 'wb') as f:
        f.write(content)
    
    return filepath