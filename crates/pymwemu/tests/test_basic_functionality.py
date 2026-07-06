#!/usr/bin/env python3
"""
Basic functionality tests for pymwemu
Inspired by libmwemu/src/tests.rs
"""

import unittest
import pymwemu
import tempfile
import os


class TestBasicFunctionality(unittest.TestCase):
    """Test basic emulator functionality"""

    def setUp(self):
        """Set up test fixtures"""
        self.emu32 = pymwemu.init32()
        self.emu64 = pymwemu.init64()
        
    def tearDown(self):
        """Clean up after tests"""
        pass

    def test_emulator_initialization(self):
        """Test emulator initialization and basic operations"""
        # Test 32-bit emulator
        self.assertIsNotNone(self.emu32)
        self.assertTrue(self.emu32.is_32bits())
        
        # Test 64-bit emulator  
        self.assertIsNotNone(self.emu64)
        self.assertTrue(self.emu64.is_64bits())
        
        # Test verbose mode
        self.emu32.set_verbose(1)
        self.emu64.set_verbose(0)
        
        # Test banzai mode
        self.emu32.enable_banzai_mode()
        self.emu64.disable_banzai_mode()

    def test_register_operations(self):
        """Test register get/set operations"""
        # Test 32-bit registers
        self.emu32.set_reg('eax', 0x12345678)
        self.assertEqual(self.emu32.get_reg('eax'), 0x12345678)
        
        self.emu32.set_reg('ebx', 0xCAFEBABE)
        self.assertEqual(self.emu32.get_reg('ebx'), 0xCAFEBABE)
        
        self.emu32.set_reg('esp', 0x7FFF0000)
        self.assertEqual(self.emu32.get_reg('esp'), 0x7FFF0000)
        
        # Test 16-bit subregisters
        self.emu32.set_reg('ax', 0xBEEF)
        self.assertEqual(self.emu32.get_reg('ax'), 0xBEEF)
        
        # Test 8-bit subregisters
        self.emu32.set_reg('al', 0x44)
        self.assertEqual(self.emu32.get_reg('al'), 0x44)
        
        self.emu32.set_reg('ah', 0x22)
        self.assertEqual(self.emu32.get_reg('ah'), 0x22)
        
        # Test 64-bit registers
        self.emu64.set_reg('rax', 0x123456789ABCDEF0)
        self.assertEqual(self.emu64.get_reg('rax'), 0x123456789ABCDEF0)
        
        self.emu64.set_reg('rbx', 0xCAFEBABECAFEBABE)
        self.assertEqual(self.emu64.get_reg('rbx'), 0xCAFEBABECAFEBABE)

    def test_memory_operations(self):
        """Test memory allocation and read/write operations"""
        # Test memory allocation
        base = self.emu32.alloc("test_mem", 0x1000)
        self.assertIsNotNone(base)
        self.assertGreater(base, 0)
        
        # Test byte operations
        self.assertTrue(self.emu32.write_byte(base, 0xAA))
        self.assertEqual(self.emu32.read_byte(base), 0xAA)
        
        # Test word operations
        self.assertTrue(self.emu32.write_word(base + 4, 0xBEEF))
        self.assertEqual(self.emu32.read_word(base + 4), 0xBEEF)
        
        # Test dword operations
        self.assertTrue(self.emu32.write_dword(base + 8, 0xDEADBEEF))
        self.assertEqual(self.emu32.read_dword(base + 8), 0xDEADBEEF)
        
        # Test qword operations (64-bit)
        base64 = self.emu64.alloc("test_mem64", 0x1000)
        self.assertTrue(self.emu64.write_qword(base64, 0x123456789ABCDEF0))
        self.assertEqual(self.emu64.read_qword(base64), 0x123456789ABCDEF0)

    def test_memory_boundary_conditions(self):
        """Test memory boundary conditions with banzai mode"""
        # Test with banzai mode enabled
        self.emu32.enable_banzai_mode()
        base = self.emu32.alloc("boundary_test", 0x1000)
        
        # Test writing within bounds - should succeed
        self.assertTrue(self.emu32.write_dword(base, 0x12345678))
        self.assertEqual(self.emu32.read_dword(base), 0x12345678)
        
        # Test writing at exact boundary - should succeed
        self.assertTrue(self.emu32.write_dword(base + 0x1000 - 4, 0x87654321))
        self.assertEqual(self.emu32.read_dword(base + 0x1000 - 4), 0x87654321)
        
        # Test writing past boundary - should raise exception even with banzai mode
        with self.assertRaises(Exception):
            self.emu32.write_dword(base + 0x1000 - 3, 0x12345678)
        
        # Test reading from unallocated memory - should raise exception
        with self.assertRaises(Exception):
            self.emu32.read_dword(0x999999)
        
        with self.assertRaises(Exception):
            self.emu32.write_dword(0x999999, 0x12345678)

    def test_string_operations(self):
        """Test string read/write operations"""
        base = self.emu32.alloc("string_test", 0x1000)
        
        # Test ASCII string operations
        test_string = "Hello, World!"
        self.emu32.write_string(base, test_string)
        read_string = self.emu32.read_string(base)
        self.assertEqual(read_string, test_string)
        
        # Test wide string operations
        wide_test = "Wide String Test"
        self.emu32.write_wide_string(base + 100, wide_test)
        read_wide = self.emu32.read_wide_string(base + 100)
        self.assertEqual(read_wide, wide_test)

    def test_memory_search_operations(self):
        """Test memory search functionality"""
        base = self.emu32.alloc("search_test", 0x1000)
        
        # Write test pattern
        test_pattern = "DEADBEEF"
        self.emu32.write_string(base + 0x100, test_pattern)
        
        # Search for the pattern
        results = self.emu32.search_string(test_pattern, "search_test")
        self.assertIsNotNone(results)
        self.assertIn(base + 0x100, results)

    def test_stack_operations(self):
        """Test stack push/pop operations"""
        # Set up stack
        stack_base = self.emu32.alloc("stack", 0x1000)
        self.emu32.set_reg('esp', stack_base + 0x800)
        
        # Test push operation
        test_value = 0x12345678
        self.assertTrue(self.emu32.stack_push32(test_value))
        
        # Verify stack pointer moved
        new_esp = self.emu32.get_reg('esp')
        self.assertEqual(new_esp, stack_base + 0x800 - 4)
        
        # Verify value was written to stack
        self.assertEqual(self.emu32.read_dword(new_esp), test_value)
        
        # Test pop operation
        popped_value = self.emu32.stack_pop32()
        self.assertEqual(popped_value, test_value)
        
        # Verify stack pointer restored
        self.assertEqual(self.emu32.get_reg('esp'), stack_base + 0x800)

    def test_error_conditions(self):
        """Test error conditions and edge cases"""
        # Test invalid register names
        with self.assertRaises(Exception):
            self.emu32.get_reg('invalid_register')
        
        # Test reading from address 0
        self.assertEqual(self.emu32.read_string(0), "")
        
        # Test memory operations on unallocated memory without banzai
        self.emu32.disable_banzai_mode()
        # These should raise exceptions
        with self.assertRaises(Exception):
            self.emu32.read_byte(0x999999)

    def test_configuration_management(self):
        """Test configuration settings"""
        # Test verbose mode
        self.emu32.set_verbose(2)
        
        # Test console output control
        self.emu32.enable_colors()
        self.emu32.disable_colors()
        
        # Test instruction counting
        initial_count = self.emu32.get_position()
        self.assertGreaterEqual(initial_count, 0)

    def test_memory_map_operations(self):
        """Test memory map operations and edge cases"""
        # Test multiple allocations
        base1 = self.emu32.alloc("map1", 0x1000)
        base2 = self.emu32.alloc("map2", 0x1000)
        base3 = self.emu32.alloc("map3", 0x1000)
        
        self.assertNotEqual(base1, base2)
        self.assertNotEqual(base2, base3)
        self.assertNotEqual(base1, base3)
        
        # Test cross-map operations
        self.assertTrue(self.emu32.write_dword(base1, 0x11111111))
        self.assertTrue(self.emu32.write_dword(base2, 0x22222222))
        self.assertTrue(self.emu32.write_dword(base3, 0x33333333))
        
        self.assertEqual(self.emu32.read_dword(base1), 0x11111111)
        self.assertEqual(self.emu32.read_dword(base2), 0x22222222)
        self.assertEqual(self.emu32.read_dword(base3), 0x33333333)
        
        # Test memory copy between maps (manual implementation)
        # Read from source and write to destination
        value = self.emu32.read_dword(base1)
        self.assertTrue(self.emu32.write_dword(base2 + 4, value))
        self.assertEqual(self.emu32.read_dword(base2 + 4), 0x11111111)


if __name__ == '__main__':
    unittest.main()