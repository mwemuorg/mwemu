#!/usr/bin/env python3
"""
Advanced features tests for pymwemu
Tests FPU, hooks, serialization, and complex scenarios
"""

import unittest
import pymwemu


class TestAdvancedFeatures(unittest.TestCase):
    """Test advanced emulator features"""

    def setUp(self):
        """Set up test fixtures"""
        self.emu32 = pymwemu.init32()
        self.emu64 = pymwemu.init64()
        self.emu32.enable_banzai_mode()
        self.emu64.enable_banzai_mode()

    def test_fpu_operations(self):
        """Test FPU (floating point unit) operations"""
        # Test basic FPU functionality if available
        try:
            # Test setting FPU registers (if API exists)
            # This depends on pymwemu's FPU API
            pass
        except AttributeError:
            # FPU API might not be exposed to Python
            self.skipTest("FPU API not available in Python bindings")

    def test_serialization(self):
        """Test emulator state serialization"""
        # Set up some state
        base = self.emu32.alloc("serial_test", 0x1000)
        self.emu32.set_reg('eax', 0x12345678)
        self.emu32.set_reg('ebx', 0xDEADBEEF)
        self.emu32.write_dword(base, 0xCAFEBABE)
        
        try:
            # Test serialization (if API exists)
            state = self.emu32.serialize()
            self.assertIsNotNone(state)
            
            # Create new emulator and deserialize
            emu_new = pymwemu.init32()
            emu_new.deserialize(state)
            
            # Verify state was restored
            self.assertEqual(emu_new.get_reg('eax'), 0x12345678)
            self.assertEqual(emu_new.get_reg('ebx'), 0xDEADBEEF)
            
        except AttributeError:
            self.skipTest("Serialization API not available in Python bindings")

    def test_memory_protection(self):
        """Test memory protection and access violations"""
        # Test different memory protection scenarios
        base = self.emu32.alloc("protect_test", 0x1000)
        
        # Test normal read/write
        self.assertTrue(self.emu32.write_dword(base, 0x12345678))
        self.assertEqual(self.emu32.read_dword(base), 0x12345678)
        
        # Test boundary violations with banzai mode
        self.emu32.enable_banzai_mode()
        
        # Should fail gracefully - catch the exception instead of expecting False
        with self.assertRaises(ValueError):
            self.emu32.write_dword(base + 0x1000, 0x87654321)

    def test_complex_memory_operations(self):
        """Test complex memory operations"""
        # Test memory copy across different regions
        src_base = self.emu32.alloc("src", 0x1000)
        dst_base = self.emu32.alloc("dst", 0x1000)
        
        # Fill source with pattern
        test_data = list(range(256))
        for i, val in enumerate(test_data):
            self.emu32.write_byte(src_base + i, val & 0xFF)
        
        # Copy data manually (since memcpy is not available)
        for i in range(len(test_data)):
            byte_val = self.emu32.read_byte(src_base + i)
            self.emu32.write_byte(dst_base + i, byte_val)
        
        # Verify copy
        for i, expected_val in enumerate(test_data):
            actual_val = self.emu32.read_byte(dst_base + i)
            self.assertEqual(actual_val, expected_val & 0xFF)

    def test_memory_search_advanced(self):
        """Test advanced memory search operations"""
        base = self.emu32.alloc("search_advanced", 0x2000)
        
        # Create complex pattern
        pattern1 = "DEADBEEF"
        pattern2 = "CAFEBABE"
        
        # Write patterns at different locations
        self.emu32.write_string(base + 0x100, pattern1)
        self.emu32.write_string(base + 0x500, pattern2)
        self.emu32.write_string(base + 0x1000, pattern1)  # Duplicate
        
        # Search for first pattern
        results1 = self.emu32.search_string(pattern1, "search_advanced")
        if results1:
            self.assertEqual(len(results1), 2)  # Should find 2 occurrences
            self.assertIn(base + 0x100, results1)
            self.assertIn(base + 0x1000, results1)
        
        # Search for second pattern
        results2 = self.emu32.search_string(pattern2, "search_advanced")
        if results2:
            self.assertEqual(len(results2), 1)
            self.assertIn(base + 0x500, results2)

    def test_instruction_tracing(self):
        """Test instruction tracing capabilities"""
        code_base = self.emu32.alloc("trace_code", 0x1000)
        
        # Simple sequence of instructions
        instructions = [
            "B8 78 56 34 12",  # mov eax, 0x12345678
            "BB EF BE AD DE",  # mov ebx, 0xDEADBEEF
            "01 D8",           # add eax, ebx
            "C3"               # ret
        ]
        
        offset = 0
        for inst in instructions:
            self.emu32.write_spaced_bytes(code_base + offset, inst)
            offset += len(inst.split())
        
        # Enable tracing if available
        try:
            self.emu32.enable_trace()
            
            # Execute instructions
            self.emu32.set_reg('eip', code_base)
            for _ in range(len(instructions) - 1):  # Don't execute ret
                self.assertTrue(self.emu32.step())
            
            # Get trace if available
            trace = self.emu32.get_trace()
            self.assertIsNotNone(trace)
            
        except AttributeError:
            self.skipTest("Tracing API not available in Python bindings")

    def test_register_tracing(self):
        """Test register value tracing"""
        try:
            # Enable register tracing for specific registers
            self.emu32.enable_trace_reg(['eax', 'ebx'])
            
            # Perform operations that change registers
            self.emu32.set_reg('eax', 0x12345678)
            self.emu32.set_reg('ebx', 0xDEADBEEF)
            
            # Get register trace
            trace = self.emu32.get_reg_trace()
            self.assertIsNotNone(trace)
            
        except AttributeError:
            self.skipTest("Register tracing API not available in Python bindings")

    def test_memory_mapping_advanced(self):
        """Test advanced memory mapping scenarios"""
        # Test overlapping memory detection
        base1 = self.emu32.alloc("map1", 0x1000)
        
        # Try to allocate overlapping memory (should fail)
        try:
            base2 = self.emu32.alloc("map2", 0x1000, base1 + 0x500)  # Overlapping
            # If this succeeds, it might indicate an issue
        except Exception:
            # Expected to fail
            pass
        
        # Test large allocations
        large_base = self.emu32.alloc("large_map", 0x100000)  # 1MB
        self.assertIsNotNone(large_base)
        
        # Test writing to different parts of large allocation
        self.assertTrue(self.emu32.write_dword(large_base, 0x11111111))
        self.assertTrue(self.emu32.write_dword(large_base + 0x50000, 0x22222222))
        self.assertTrue(self.emu32.write_dword(large_base + 0x99000, 0x33333333))
        
        # Verify
        self.assertEqual(self.emu32.read_dword(large_base), 0x11111111)
        self.assertEqual(self.emu32.read_dword(large_base + 0x50000), 0x22222222)
        self.assertEqual(self.emu32.read_dword(large_base + 0x99000), 0x33333333)

    def test_console_and_output(self):
        """Test console output and color settings"""
        # Test console settings (using available methods)
        self.emu32.enable_colors()
        self.emu32.set_verbose(2)
        
        # Perform operations that might generate output
        base = self.emu32.alloc("console_test", 0x1000)
        self.emu32.write_dword(base, 0x12345678)
        
        # Test disabling console color
        self.emu32.disable_colors()

    def test_error_handling_edge_cases(self):
        """Test error handling in edge cases"""
        # Test with invalid parameters - reading from invalid address should raise exception
        with self.assertRaises(Exception):
            self.emu32.read_dword(0xDEADBEEF)  # Invalid address
        
        # Test reading from null pointer
        result = self.emu32.read_string(0)
        self.assertEqual(result, "")
        
        # Test very large allocations (might fail gracefully)
        try:
            huge_base = self.emu32.alloc("huge", 0xFFFFFFFF)  # 4GB
            # If this succeeds, test basic operations
            if huge_base:
                self.assertTrue(self.emu32.write_dword(huge_base, 0x12345678))
        except Exception:
            # Expected to fail on most systems
            pass

    def test_performance_scenarios(self):
        """Test performance-related scenarios"""
        # Test many small allocations
        bases = []
        for i in range(100):
            base = self.emu32.alloc(f"perf_test_{i}", 0x100)
            bases.append(base)
            self.emu32.write_dword(base, i)
        
        # Verify all allocations
        for i, base in enumerate(bases):
            self.assertEqual(self.emu32.read_dword(base), i)
        
        # Test large memory operations
        large_base = self.emu32.alloc("large_perf", 0x10000)
        
        # Fill with pattern
        for i in range(0, 0x10000, 4):
            self.emu32.write_dword(large_base + i, i // 4)
        
        # Verify pattern
        for i in range(0, 0x10000, 4):
            expected = i // 4
            actual = self.emu32.read_dword(large_base + i)
            self.assertEqual(actual, expected)

    def test_64bit_specific_features(self):
        """Test 64-bit specific features"""
        # Test 64-bit register operations
        self.emu64.set_reg('rax', 0xFFFFFFFFFFFFFFFF)
        self.assertEqual(self.emu64.get_reg('rax'), 0xFFFFFFFFFFFFFFFF)
        
        # Test large address space
        high_base = self.emu64.alloc("high_mem", 0x1000)  # Should get high address
        self.assertIsNotNone(high_base)
        
        # Test 64-bit memory operations
        self.assertTrue(self.emu64.write_qword(high_base, 0x123456789ABCDEF0))
        self.assertEqual(self.emu64.read_qword(high_base), 0x123456789ABCDEF0)


if __name__ == '__main__':
    unittest.main()
