//! Bridge: implement rs-header's `ElfLoader` for our memory model (`Maps`), so
//! the generic ELF loader maps segments/sections and patches relocations
//! directly into *this* emulator's address space. All ELF parsing stays in
//! rs-header; this only provides the "how to touch memory" half.

use rs_header::elf::{ElfLoader, Perm};

use crate::maps::Maps;
use crate::maps::mem64::Permission;

fn to_permission(p: Perm) -> Permission {
    Permission::from_flags(p.read, p.write, p.execute)
}

impl ElfLoader for Maps {
    fn map(&mut self, name: &str, addr: u64, size: u64, perm: Perm) -> Option<u64> {
        let permission = to_permission(perm);
        match self.create_map(name, addr, size, permission) {
            Ok(_) => Some(addr),
            Err(_) => {
                // Overlap (or otherwise unmappable) at the requested address —
                // relocate the region elsewhere, mirroring the old loader.
                let relocated = self.alloc(size + 10)?;
                self.create_map(name, relocated, size, permission).ok()?;
                Some(relocated)
            }
        }
    }

    fn lib_alloc(&mut self, size: u64) -> Option<u64> {
        self.lib64_alloc(size)
    }

    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        match self.get_mem_by_addr_mut(addr) {
            Some(mem) => {
                mem.force_write_bytes(addr, data);
                true
            }
            None => false,
        }
    }

    fn read_qword(&self, addr: u64) -> Option<u64> {
        Maps::read_qword(self, addr)
    }

    fn write_qword(&mut self, addr: u64, val: u64) -> bool {
        // Force the write so relocations can patch read-only segments (GOT,
        // RELRO) just like ld.so does before re-protecting them.
        match self.get_mem_by_addr_mut(addr) {
            Some(mem) => {
                mem.force_write_qword(addr, val);
                true
            }
            None => false,
        }
    }
}
