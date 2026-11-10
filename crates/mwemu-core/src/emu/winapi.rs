use crate::utils::helpers::unlikely;
use crate::{
    api::dispatch,
    emu::{API_CALL_LOG_CAP, ApiCallLogEntry, Emu},
    exception::types::ExceptionType,
    windows::native_crt,
};

impl Emu {
    /// Record a resolved call into `api_call_log` (see `engine/instructions/call.rs`,
    /// gated by `cfg.trace_calls`). Drops the oldest entry once `API_CALL_LOG_CAP`
    /// is reached, so a long trace stays bounded.
    pub fn log_api_call(&mut self, pos: u64, from: u64, to: u64, name: String) {
        if self.api_call_log.len() >= API_CALL_LOG_CAP {
            self.api_call_log.pop_front();
        }
        self.api_call_log.push_back(ApiCallLogEntry {
            pos,
            from,
            to,
            name,
        });
    }

    //TODO: check this, this is used only on pymwemu
    /// Call a winapi by addess.
    pub fn handle_winapi(&mut self, addr: u64) {
        if self.cfg.arch.is_64bits() {
            let name = match self.maps.get_addr_name(addr) {
                Some(n) => n,
                None => {
                    log::error!("/!\\ setting pc to non mapped addr 0x{:x}", addr);
                    self.exception(ExceptionType::SettingRipToNonMappedAddr);
                    return;
                }
            };
            if unlikely(native_crt::is_native_section(name)) {
                // Match the engine-side path: native msvcrt bytes own their
                // own `ret`. The caller has already pushed the return address;
                // do not consume it here.
                native_crt::log_native_call(self, addr);
                self.set_pc(addr);
                return;
            }
            let temp_name = name.to_string();
            self.gateway_return = self.stack_pop64(false).unwrap_or(0);
            self.set_pc(self.gateway_return);
            dispatch::winapi64_gateway(addr, temp_name.as_str(), self);
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
            dispatch::winapi32_gateway(addr as u32, name.to_string().as_str(), self);
        }
    }

    /// For an existing linked DLL, this funcion allows to modify the base address on LDR entry.
    pub fn update_ldr_entry_base(&mut self, libname: &str, base: u64) {
        if self.cfg.arch.is_64bits() {
            crate::layers::windows().update_ldr_entry_base(self, libname, base, true);
        } else {
            crate::layers::windows().update_ldr_entry_base(self, libname, base, false);
        }
    }

    /// Dynamic link a windows DLL from emu.cfg.maps_folder.
    pub fn link_library(&mut self, libname: &str) -> u64 {
        if self.cfg.arch.is_64bits() {
            crate::layers::windows().load_library(self, libname, true)
        } else {
            crate::layers::windows().load_library(self, libname, false)
        }
    }

    /// Resolve the winapi name having an address.
    pub fn api_addr_to_name(&mut self, addr: u64) -> String {
        let name: String = if self.cfg.arch.is_64bits() {
            crate::layers::windows().resolve_api_addr_to_name(self, addr, true)
        } else {
            crate::layers::windows().resolve_api_addr_to_name(self, addr, false)
        };

        name
    }

    /// Resolve the address of an api name keyword.
    pub fn api_name_to_addr(&mut self, kw: &str) -> u64 {
        if self.cfg.arch.is_64bits() {
            let (addr, lib, name) = crate::layers::windows().search_api_name(self, kw, true);
            addr
        } else {
            let (addr, lib, name) = crate::layers::windows().search_api_name(self, kw, false);
            addr
        }
    }
}
