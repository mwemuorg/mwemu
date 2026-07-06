use crate::{
    emu::Emu,
    exception::types::ExceptionType,
    winapi::{winapi32, winapi64},
    windows::peb::{peb32, peb64},
};

impl Emu {
    //TODO: check this, this is used only on pymwemu
    /// Call a winapi by addess.
    pub fn handle_winapi(&mut self, addr: u64) {
        if self.cfg.arch.is_64bits() {
            self.gateway_return = self.stack_pop64(false).unwrap_or(0);
            self.set_pc(self.gateway_return);
            let name = match self.maps.get_addr_name(addr) {
                Some(n) => n,
                None => {
                    log::error!("/!\\ setting pc to non mapped addr 0x{:x}", addr);
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    return;
                }
            };
            winapi64::gateway(addr, name.to_string().as_str(), self);
        } else {
            self.gateway_return = self.stack_pop32(false).unwrap_or(0) as u64;
            self.set_pc(self.gateway_return);
            let name = match self.maps.get_addr_name(addr) {
                Some(n) => n,
                None => {
                    log::error!("/!\\ setting pc to non mapped addr 0x{:x}", addr);
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    return;
                }
            };
            winapi32::gateway(addr as u32, name.to_string().as_str(), self);
        }
    }

    /// For an existing linked DLL, this funcion allows to modify the base address on LDR entry.
    pub fn update_ldr_entry_base(&mut self, libname: &str, base: u64) {
        if self.cfg.arch.is_64bits() {
            peb64::update_ldr_entry_base(libname, base, self);
        } else {
            peb32::update_ldr_entry_base(libname, base, self);
        }
    }

    /// Dynamic link a windows DLL from emu.cfg.maps_folder.
    pub fn link_library(&mut self, libname: &str) -> u64 {
        if self.cfg.arch.is_64bits() {
            winapi64::kernel32::load_library(self, libname)
        } else {
            winapi32::kernel32::load_library(self, libname)
        }
    }

    /// Resolve the winapi name having an address.
    pub fn api_addr_to_name(&mut self, addr: u64) -> String {
        let name: String = if self.cfg.arch.is_64bits() {
            winapi64::kernel32::resolve_api_addr_to_name(self, addr)
        } else {
            winapi32::kernel32::resolve_api_addr_to_name(self, addr)
        };

        name
    }

    /// Resolve the address of an api name keyword.
    pub fn api_name_to_addr(&mut self, kw: &str) -> u64 {
        if self.cfg.arch.is_64bits() {
            let (addr, lib, name) = winapi64::kernel32::search_api_name(self, kw);
            addr
        } else {
            let (addr, lib, name) = winapi32::kernel32::search_api_name(self, kw);
            addr
        }
    }
}
