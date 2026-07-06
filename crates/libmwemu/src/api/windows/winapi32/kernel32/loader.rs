use crate::api::windows::common::kernel32 as kernel32_common;
use crate::emu;
use crate::windows::peb::peb32;

pub fn load_library(emu: &mut emu::Emu, libname: &str) -> u64 {
    let Some(dll) = kernel32_common::normalize_library_name(libname, true) else {
        emu.regs_mut().rax = 0;
        return 0;
    };

    // API set DLLs are virtual; map them to a real provider to avoid hard failure.
    if kernel32_common::is_api_set_contract(&dll) {
        let base = load_library(emu, "kernelbase.dll");
        if base != 0 {
            return base;
        }
        return load_library(emu, "kernel32.dll");
    }

    let mut dll_path = emu.cfg.maps_folder.clone();
    dll_path.push('/');
    dll_path.push_str(&dll);

    match peb32::get_module_base(&dll, emu) {
        Some(base) => base,
        None => {
            // Safety net: fetch from the symbol server if missing (no-op until
            // winver gains 32-bit support; harmless meanwhile).
            emu.ensure_maps_dll(&dll);
            if std::path::Path::new(dll_path.as_str()).exists() {
                let (base, pe_off) = emu.load_pe32(&dll_path, false, 0);
                peb32::dynamic_link_module(base as u64, pe_off, &dll, emu);
                emu.library_loaded = true; // Signal to GDB that library list changed
                base as u64
            } else {
                log::trace!("dll {} not found.", dll_path);
                0
            }
        }
    }
}
