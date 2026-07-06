//! Bridge: implement mwemu-pe's `PeLoader` for `Emu`, so the generic PE loader
//! patches *this* emulator's memory and resolves imports through *our* winapi.
//! Arch (32/64) is picked from `cfg`.

use rs_header::pe::PeLoader;

use crate::emu::Emu;
use crate::winapi::{winapi32, winapi64};

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
        if self.cfg.is_x64() {
            winapi64::kernel32::load_library(self, libname)
        } else {
            winapi32::kernel32::load_library(self, libname)
        }
    }

    fn resolve_api_name(&mut self, name: &str) -> u64 {
        if self.cfg.is_x64() {
            winapi64::kernel32::resolve_api_name(self, name)
        } else {
            winapi32::kernel32::resolve_api_name(self, name)
        }
    }

    fn resolve_api_name_in_module(&mut self, module: &str, name: &str) -> u64 {
        if self.cfg.is_x64() {
            winapi64::kernel32::resolve_api_name_in_module(self, module, name)
        } else {
            winapi32::kernel32::resolve_api_name_in_module(self, module, name)
        }
    }

    fn search_api_name(&mut self, name: &str) -> (u64, String, String) {
        if self.cfg.is_x64() {
            winapi64::kernel32::search_api_name(self, name)
        } else {
            winapi32::kernel32::search_api_name(self, name)
        }
    }
}
