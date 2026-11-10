//! Bridge: implement mwemu-pe's `PeLoader` for `Emu`, so the generic PE loader
//! patches *this* emulator's memory and resolves imports through *our* winapi.
//! Arch (32/64) is picked from `cfg`.

use rs_header::pe::PeLoader;

use crate::emu::Emu;

impl PeLoader for Emu {
    fn is_mapped(&self, addr: u64) -> bool {
        self.maps.is_mapped(addr)
    }

    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> bool {
        self.maps.write_bytes(addr, data)
    }

    fn write_dword(&mut self, addr: u64, val: u32) -> bool {
        match self.maps.get_mem_by_addr_mut(addr) {
            Some(mem) => {
                mem.force_write_dword(addr, val);
                true
            }
            None => false,
        }
    }

    fn write_qword(&mut self, addr: u64, val: u64) -> bool {
        match self.maps.get_mem_by_addr_mut(addr) {
            Some(mem) => {
                mem.force_write_qword(addr, val);
                true
            }
            None => false,
        }
    }

    fn load_library(&mut self, libname: &str) -> u64 {
        let is64 = self.cfg.is_x64();
        crate::layers::windows().load_library(self, libname, is64)
    }

    fn resolve_api_name(&mut self, name: &str) -> u64 {
        let is64 = self.cfg.is_x64();
        crate::layers::windows().resolve_api_name(self, name, is64)
    }

    fn resolve_api_name_in_module(&mut self, module: &str, name: &str) -> u64 {
        let is64 = self.cfg.is_x64();
        crate::layers::windows().resolve_api_name_in_module(self, module, name, is64)
    }

    fn resolve_api_ordinal_in_module(&mut self, module: &str, ordinal: u16) -> u64 {
        // Module-scoped ordinal resolution goes through the export registry
        // that `Emu::load_pe64` / `load_pe32` populate for every mapped
        // image. The result is rebased to the guest address; zero is the
        // signal that the ordinal was not exported (or the module is
        // missing), which the binder treats as "leave the slot untouched".
        self.export_indexes
            .resolve_ordinal_in_module(module, ordinal as u32)
    }

    fn search_api_name(&mut self, name: &str) -> (u64, String, String) {
        let is64 = self.cfg.is_x64();
        crate::layers::windows().search_api_name(self, name, is64)
    }
}
