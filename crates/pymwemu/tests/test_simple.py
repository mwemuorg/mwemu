#!/usr/bin/env python3
"""
Simple working tests for pymwemu using only available API methods
"""

import unittest
import pymwemu


class TestSimpleFunctionality(unittest.TestCase):
    """Simple tests that actually work with the real pymwemu API"""

    def setUp(self):
        """Set up test fixtures"""
        self.emu32 = pymwemu.init32()
        self.emu64 = pymwemu.init64()
        self.emu32.enable_banzai_mode()
        self.emu64.enable_banzai_mode()

    def test_emulator_creation(self):
        """Test basic emulator creation"""
        self.assertIsNotNone(self.emu32)
        self.assertIsNotNone(self.emu64)
        self.assertTrue(self.emu32.is_32bits())
        self.assertTrue(self.emu64.is_64bits())

    def test_register_operations(self):
        """Test register operations"""
        # Test 32-bit registers
        self.emu32.set_reg('eax', 0x12345678)
        self.assertEqual(self.emu32.get_reg('eax'), 0x12345678)
        
        self.emu32.set_reg('ebx', 0xDEADBEEF)
        self.assertEqual(self.emu32.get_reg('ebx'), 0xDEADBEEF)
        
        # Test 64-bit registers
        self.emu64.set_reg('rax', 0x123456789ABCDEF0)
        self.assertEqual(self.emu64.get_reg('rax'), 0x123456789ABCDEF0)

    def test_memory_allocation(self):
        """Test memory allocation"""
        base = self.emu32.alloc("test_mem", 0x1000)
        self.assertIsNotNone(base)
        self.assertGreater(base, 0)

    def test_memory_read_write(self):
        """Test memory read/write operations"""
        base = self.emu32.alloc("rw_test", 0x1000)
        
        # Test byte operations
        self.assertTrue(self.emu32.write_byte(base, 0xAA))
        self.assertEqual(self.emu32.read_byte(base), 0xAA)
        
        # Test word operations
        self.assertTrue(self.emu32.write_word(base + 4, 0xBEEF))
        self.assertEqual(self.emu32.read_word(base + 4), 0xBEEF)
        
        # Test dword operations
        self.assertTrue(self.emu32.write_dword(base + 8, 0xDEADBEEF))
        self.assertEqual(self.emu32.read_dword(base + 8), 0xDEADBEEF)

    def test_string_operations(self):
        """Test string operations"""
        base = self.emu32.alloc("string_test", 0x1000)
        
        test_string = "Hello, World!"
        self.emu32.write_string(base, test_string)
        read_string = self.emu32.read_string(base)
        self.assertEqual(read_string, test_string)

    def test_stack_operations(self):
        """Test stack operations"""
        stack_base = self.emu32.alloc("stack", 0x1000)
        self.emu32.set_reg('esp', stack_base + 0x800)
        
        test_value = 0x12345678
        self.assertTrue(self.emu32.stack_push32(test_value))
        
        # Check stack pointer moved
        new_esp = self.emu32.get_reg('esp')
        self.assertEqual(new_esp, stack_base + 0x800 - 4)
        
        # Pop and verify
        popped_value = self.emu32.stack_pop32()
        self.assertEqual(popped_value, test_value)

    def test_buffer_operations(self):
        """Test buffer read/write operations"""
        base = self.emu32.alloc("buffer_test", 0x1000)
        
        test_data = bytes([0x41, 0x42, 0x43, 0x44])  # "ABCD" as bytes
        self.emu32.write_buffer(base, test_data)
        
        read_data = self.emu32.read_buffer(base, len(test_data))
        self.assertEqual(bytes(read_data), test_data)

    def test_search_operations(self):
        """Test memory search operations"""
        base = self.emu32.alloc("search_test", 0x1000)
        
        # Write test pattern
        test_string = "DEADBEEF"
        self.emu32.write_string(base + 0x100, test_string)
        
        # Search for the pattern
        results = self.emu32.search_string(test_string, "search_test")
        self.assertIsInstance(results, list)
        if results:  # If found
            self.assertIn(base + 0x100, results)

    def test_spaced_bytes(self):
        """Test spaced bytes operations"""
        base = self.emu32.alloc("bytes_test", 0x1000)
        
        # Write spaced bytes
        hex_bytes = "41 42 43 44"  # "ABCD"
        self.assertTrue(self.emu32.write_spaced_bytes(base, hex_bytes))
        
        # Verify by reading as string
        result = self.emu32.read_string(base)
        self.assertEqual(result, "ABCD")

    def test_configuration(self):
        """Test configuration methods"""
        # Test verbose levels
        self.emu32.set_verbose(0)
        self.emu32.set_verbose(1)
        self.emu32.set_verbose(2)
        
        # Test color settings
        self.emu32.enable_colors()
        self.emu32.disable_colors()
        
        # Test tracing
        self.emu32.enable_trace_mem()
        self.emu32.disable_trace_mem()

    def test_basic_execution(self):
        """Test basic instruction execution"""
        code_base = self.emu32.alloc("code", 0x1000)
        
        # Simple NOP instruction (0x90)
        self.emu32.write_byte(code_base, 0x90)
        
        # Set instruction pointer
        self.emu32.set_reg('eip', code_base)
        
        # Execute one step
        result = self.emu32.step()
        self.assertTrue(result)

    def test_position_tracking(self):
        """Test instruction position tracking"""
        initial_pos = self.emu32.get_position()
        self.assertGreaterEqual(initial_pos, 0)
        
        # Reset position
        self.emu32.reset_pos()
        self.assertEqual(self.emu32.get_position(), 0)

    def test_memory_info(self):
        """Test memory information methods"""
        base = self.emu32.alloc("info_test", 0x1000)
        
        # Test if mapped
        self.assertTrue(self.emu32.is_mapped(base))
        
        # Test address name
        name = self.emu32.get_addr_name(base)
        self.assertEqual(name, "info_test")
        
        # Test allocated size
        size = self.emu32.allocated_size()
        self.assertGreater(size, 0)

    def test_error_handling(self):
        """Test error handling"""
        # Test invalid register
        with self.assertRaises(Exception):
            self.emu32.get_reg('invalid_register')
        
        # Test reading from unallocated memory
        with self.assertRaises(Exception):
            self.emu32.read_dword(0x999999)


if __name__ == '__main__':
    unittest.main()
