use crate::api::windows::common::modules;
use crate::emu;
use crate::emu::loaders::pe_stub_exports::get_stub_exports;
use crate::windows::peb::peb32;

pub fn load_library(emu: &mut emu::Emu, libname: &str) -> u64 {
    let Some(dll) = modules::normalize_library_name(libname, true) else {
        emu.regs_mut().rax = 0;
        return 0;
    };

    // API set DLLs are virtual; map them to a real provider to avoid hard failure.
    if modules::is_api_set_contract(&dll) {
        let base = load_library(emu, "kernelbase.dll");
        if base != 0 {
            return base;
        }
        return load_library(emu, "kernel32.dll");
    }

    let dll_path = emu.cfg.get_maps_folder(&dll);

    match peb32::get_module_base(&dll, emu) {
        Some(base) => base,
        None => {
            // Safety net: fetch from the symbol server if missing (no-op until
            // winver gains 32-bit support; harmless meanwhile).
            emu.ensure_maps_dll(&dll);
            if std::path::Path::new(dll_path.as_str()).exists() {
                let image = match emu.load_pe32(&dll_path, false, 0) {
                    Ok(image) => image,
                    Err(error) => {
                        log::error!("cannot load library {dll}: {error}");
                        return 0;
                    }
                };
                peb32::dynamic_link_module(image.base, image.header_offset, &dll, emu);
                emu.library_loaded = true; // Signal to GDB that library list changed
                image.base
            } else if let Some(exports) = get_stub_exports(&dll) {
                let base = emu.create_stub_pe32(&dll, exports) as u64;
                if base != 0 {
                    peb32::dynamic_link_module(base, 0, &dll, emu);
                    emu.library_loaded = true;
                    log::info!("loaded stub PE32 for {}", dll);
                }
                base
            } else {
                log::trace!("dll {} not found.", dll_path);
                0
            }
        }
    }
}
