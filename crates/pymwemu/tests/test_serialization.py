#!/usr/bin/env python3
"""
Test the serialization functionality for pymwemu
"""

import unittest
import pymwemu
import tempfile
import os

class TestSerialization(unittest.TestCase):
    def setUp(self):
        self.emu = pymwemu.init32()
        self.emu.enable_banzai_mode()
        self.temp_dir = tempfile.mkdtemp()
        
    def tearDown(self):
        import shutil
        shutil.rmtree(self.temp_dir)

    def setup_test_state(self):
        # Set some registers
        self.emu.set_reg('eax', 0x12345678)
        self.emu.set_reg('ebx', 0xDEADBEEF)
        self.emu.set_reg('eip', 0x08048000)
        
        # Allocate and write memory
        base = self.emu.alloc("test_ser", 0x2000)
        self.emu.write_dword(base + 0x100, 0xCAFEBABE)
        return base

    def verify_test_state(self, emu_instance, base):
        self.assertEqual(emu_instance.get_reg('eax'), 0x12345678)
        self.assertEqual(emu_instance.get_reg('ebx'), 0xDEADBEEF)
        self.assertEqual(emu_instance.get_reg('eip'), 0x08048000)
        self.assertEqual(emu_instance.read_dword(base + 0x100), 0xCAFEBABE)

    def test_in_memory_serialization(self):
        """Test serialize() and deserialize() to/from bytes"""
        base = self.setup_test_state()
        
        # Serialize to bytes
        data = self.emu.serialize()
        self.assertTrue(len(data) > 0)
        
        # Modify the original to ensure it's loaded from data
        self.emu.set_reg('eax', 0x0)
        self.emu.write_dword(base + 0x100, 0x0)
        
        # Deserialize to new emu instance
        new_emu = pymwemu.deserialize(data)
        
        # Verify the state was fully recovered
        self.verify_test_state(new_emu, base)

    def test_file_serialization(self):
        """Test dump_to_file() and load_from_file()"""
        base = self.setup_test_state()
        
        filepath = os.path.join(self.temp_dir, "test.dump")
        
        # Dump to file
        self.emu.dump_to_file(filepath)
        self.assertTrue(os.path.exists(filepath))
        
        # Clear original
        self.emu.set_reg('eax', 0x0)
        
        # Load from file
        new_emu = pymwemu.load_from_file(filepath)
        
        self.verify_test_state(new_emu, base)

    def test_minidump_serialization(self):
        """Test dump_to_minidump() and load_from_minidump()"""
        base = self.setup_test_state()
        
        filepath = os.path.join(self.temp_dir, "test.dmp")
        
        # Dump to minidump
        self.emu.dump_to_minidump(filepath)
        self.assertTrue(os.path.exists(filepath))
        
        # Clear original
        self.emu.set_reg('eax', 0x0)
        
        # Load from minidump
        new_emu = pymwemu.load_from_minidump(filepath)
        
        self.verify_test_state(new_emu, base)
        
    def test_load_serialization(self):
        """Test load()"""
        base = self.setup_test_state()
        
        filepath = os.path.join(self.temp_dir, "test.dump")
        
        # Dump to file
        self.emu.dump_to_file(filepath)
        self.assertTrue(os.path.exists(filepath))
        
        # Clear original
        self.emu.set_reg('eax', 0x0)
        
        # Load from file
        new_emu = pymwemu.load(filepath)
        
        self.verify_test_state(new_emu, base)
        
    def test_dump_serialization(self):
        """Test dump()"""
        base = self.setup_test_state()
        
        filepath = os.path.join(self.temp_dir, "test.dump")
        
        # Dump to file
        self.emu.dump(filepath)
        self.assertTrue(os.path.exists(filepath))
        
        # Clear original
        self.emu.set_reg('eax', 0x0)
        
        # Load from file
        new_emu = pymwemu.load(filepath)
        
        self.verify_test_state(new_emu, base)

if __name__ == '__main__':
    unittest.main()