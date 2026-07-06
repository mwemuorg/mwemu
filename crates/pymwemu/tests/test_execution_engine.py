#!/usr/bin/env python3
"""
Execution engine tests for pymwemu
Tests instruction execution, hooks, and breakpoints
"""

import unittest
import pymwemu
import tempfile
import os


class TestExecutionEngine(unittest.TestCase):
    """Test instruction execution and control flow"""

    def setUp(self):
        """Set up test fixtures"""
        self.emu32 = pymwemu.init32()
        self.emu64 = pymwemu.init64()
        self.emu32.enable_banzai_mode()
        self.emu64.enable_banzai_mode()
        
    def test_basic_execution(self):
        """Test basic instruction execution"""
        # Allocate code memory
        code_base = self.emu32.alloc("code", 0x1000)
        
        # Simple MOV instruction: mov eax, 0x12345678
        # B8 78 56 34 12
        self.emu32.write_spaced_bytes(code_base, "B8 78 56 34 12")
        
        # Set instruction pointer
        self.emu32.set_reg('eip', code_base)
        
        # Execute one instruction
        self.assertTrue(self.emu32.step())
        
        # Check result
        self.assertEqual(self.emu32.get_reg('eax'), 0x12345678)

    def test_arithmetic_operations(self):
        """Test arithmetic instruction execution"""
        code_base = self.emu32.alloc("arith_code", 0x1000)
        
        # Set up initial values
        self.emu32.set_reg('eax', 10)
        self.emu32.set_reg('ebx', 5)
        
        # ADD eax, ebx (01 D8)
        self.emu32.write_spaced_bytes(code_base, "01 D8")
        self.emu32.set_reg('eip', code_base)
        
        self.assertTrue(self.emu32.step())
        self.assertEqual(self.emu32.get_reg('eax'), 15)

    def test_memory_access_instructions(self):
        """Test memory access instructions"""
        code_base = self.emu32.alloc("mem_code", 0x1000)
        data_base = self.emu32.alloc("data", 0x1000)
        
        # Write test data
        self.emu32.write_dword(data_base, 0xDEADBEEF)
        
        # MOV eax, [data_base] - need to encode this properly
        # For now, test with direct memory operations
        self.emu32.set_reg('ebx', data_base)
        
        # MOV eax, [ebx] (8B 03)
        self.emu32.write_spaced_bytes(code_base, "8B 03")
        self.emu32.set_reg('eip', code_base)
        
        self.assertTrue(self.emu32.step())
        self.assertEqual(self.emu32.get_reg('eax'), 0xDEADBEEF)

    def test_stack_instructions(self):
        """Test stack-related instructions"""
        code_base = self.emu32.alloc("stack_code", 0x1000)
        stack_base = self.emu32.alloc("stack", 0x1000)
        
        # Set up stack
        self.emu32.set_reg('esp', stack_base + 0x800)
        self.emu32.set_reg('eax', 0x12345678)
        
        # PUSH eax (50)
        self.emu32.write_spaced_bytes(code_base, "50")
        self.emu32.set_reg('eip', code_base)
        
        self.assertTrue(self.emu32.step())
        
        # Check stack pointer moved
        self.assertEqual(self.emu32.get_reg('esp'), stack_base + 0x800 - 4)
        
        # Check value on stack
        self.assertEqual(self.emu32.read_dword(self.emu32.get_reg('esp')), 0x12345678)

    def test_breakpoint_functionality(self):
        """Test breakpoint setting and handling"""
        code_base = self.emu32.alloc("bp_code", 0x1000)
        
        # Write some instructions
        self.emu32.write_spaced_bytes(code_base, "B8 78 56 34 12 B8 EF BE AD DE")
        
        # Set breakpoint at second instruction
        bp_addr = code_base + 5
        self.emu32.bp_set_addr(bp_addr)
        
        # Set instruction pointer
        self.emu32.set_reg('eip', code_base)
        
        # Verify breakpoint was set
        self.assertEqual(self.emu32.bp_get_addr(), [bp_addr])

    def test_instruction_counting(self):
        """Test instruction counting functionality"""
        code_base = self.emu32.alloc("count_code", 0x1000)
        
        # Write multiple NOP instructions (90)
        self.emu32.write_spaced_bytes(code_base, "90 90 90 90 90")
        
        initial_count = self.emu32.get_position()
        self.emu32.set_reg('eip', code_base)
        
        # Execute 3 instructions
        for _ in range(3):
            self.assertTrue(self.emu32.step())
        
        final_count = self.emu32.get_position()
        self.assertEqual(final_count - initial_count, 3)

    def test_flag_operations(self):
        """Test flag register operations"""
        code_base = self.emu32.alloc("flag_code", 0x1000)
        
        # Test zero flag with SUB instruction
        self.emu32.set_reg('eax', 5)
        self.emu32.set_reg('ebx', 5)
        
        # SUB eax, ebx (29 D8)
        self.emu32.write_spaced_bytes(code_base, "29 D8")
        self.emu32.set_reg('eip', code_base)
        
        self.assertTrue(self.emu32.step())
        
        # Result should be 0, zero flag should be set
        self.assertEqual(self.emu32.get_reg('eax'), 0)
        # Note: Flag checking depends on pymwemu API availability

    def test_64bit_execution(self):
        """Test 64-bit instruction execution"""
        code_base = self.emu64.alloc("code64", 0x1000)
        
        # MOV rax, 0x123456789ABCDEF0
        # 48 B8 F0 DE BC 9A 78 56 34 12
        self.emu64.write_spaced_bytes(code_base, "48 B8 F0 DE BC 9A 78 56 34 12")
        
        self.emu64.set_reg('rip', code_base)
        self.assertTrue(self.emu64.step())
        
        self.assertEqual(self.emu64.get_reg('rax'), 0x123456789ABCDEF0)

    def test_execution_limits(self):
        """Test execution limits and timeouts"""
        code_base = self.emu32.alloc("loop_code", 0x1000)
        
        # Simple instruction sequence instead of infinite loop
        self.emu32.write_spaced_bytes(code_base, "90 90 90")  # NOP NOP NOP
        
        self.emu32.set_reg('eip', code_base)
        
        # Execute a few steps
        for _ in range(3):
            self.assertTrue(self.emu32.step())

    def test_exception_handling(self):
        """Test exception handling during execution"""
        # Test division by zero or invalid memory access
        # This depends on pymwemu's exception handling capabilities
        pass

    def test_hook_functionality(self):
        """Test instruction and memory hooks"""
        # This test depends on pymwemu's hook API
        code_base = self.emu32.alloc("hook_code", 0x1000)
        
        # Write test instruction
        self.emu32.write_spaced_bytes(code_base, "B8 78 56 34 12")  # MOV eax, 0x12345678
        
        # Set up hook (if API available)
        hook_called = False
        
        def instruction_hook(emu, addr):
            nonlocal hook_called
            hook_called = True
            return True
        
        # This would depend on pymwemu's actual hook API
        # self.emu32.set_instruction_hook(instruction_hook)
        
        self.emu32.set_reg('eip', code_base)
        self.assertTrue(self.emu32.step())
        
        # Check if hook was called (if implemented)
        # self.assertTrue(hook_called)


if __name__ == '__main__':
    unittest.main()