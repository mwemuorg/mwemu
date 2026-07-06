use crate::api::windows::common::kernel32 as kernel32_common;
use crate::emu;
use crate::windows::peb::peb64;

pub fn is_library_loaded(emu: &mut emu::Emu, libname: &str) -> bool {
    let Some(dll) = kernel32_common::normalize_library_name(libname, false) else {
        return false;
    };
    peb64::get_module_base(&dll, emu).is_some()
}

pub fn load_library(emu: &mut emu::Emu, libname: &str) -> u64 {
    let Some(mut dll) = kernel32_common::normalize_library_name(libname, false) else {
        emu.regs_mut().rax = 0;
        return 0;
    };

    // API set DLL names are virtual contracts that map to host DLLs.
    if kernel32_common::is_api_set_contract(&dll) {
        if peb64::get_module_base("kernelbase.dll", emu).is_some() {
            return peb64::get_module_base("kernelbase.dll", emu).unwrap_or(0);
        }
        if peb64::get_module_base("kernel32.dll", emu).is_some() {
            return peb64::get_module_base("kernel32.dll", emu).unwrap_or(0);
        }
        dll = "kernelbase.dll".to_string();
    }

    let mut dll_path = emu.cfg.maps_folder.clone();
    dll_path.push_str(&dll);

    // Check if this DLL is already linked with a valid PE header.
    // During SSDT emulation, LdrInitializeThunk may have mapped placeholder
    // regions (from NtMapViewOfSection) that have no actual PE content.
    // We detect this by checking for the MZ magic at the reported base.
    let already_linked_base = peb64::get_module_base(&dll, emu).and_then(|base| {
        let mz = emu.maps.read_word(base).unwrap_or(0);
        if mz == 0x5A4D {
            Some(base) // valid PE: keep it
        } else {
            log::trace!(
                "load_library: {} found in LDR at 0x{:x} but has no valid PE, reloading",
                dll,
                base
            );
            None // invalid PE: force reload from file
        }
    });

    match already_linked_base {
        Some(base) => base,
        None => {
            // Guard against re-entrant loading (circular imports): if the .pe map was
            // already created by an in-progress load_pe64 call further up the stack,
            // skip loading and register the existing map as the LDR entry.
            let pe_map_name = format!("{}.pe", dll.trim_end_matches(".dll"));
            if let Some(existing) = emu.maps.get_map_by_name(&pe_map_name) {
                let base = existing.get_base();
                log::trace!(
                    "dll {} already mapped (load in progress), skipping reload",
                    dll
                );
                return base;
            }

            // Library-wide safety net: if the DLL isn't in the maps folder,
            // fetch it from the symbol server (--winver, default win11). Covers
            // PE-load-time deps, IAT/delay-load binding, runtime LoadLibrary and
            // the SSDT `load_library("ntdll")` — so no consumer hits a missing DLL.
            emu.ensure_maps_dll(&dll);

            let path = std::path::Path::new(&dll_path);
            if path.try_exists().unwrap() {
                let (base, pe_off) = emu.load_pe64(&dll_path, false, 0);
                peb64::dynamic_link_module(base, pe_off, &dll, emu);
                emu.library_loaded = true; // Signal to GDB that library list changed
                base
            } else {
                log::trace!("dll {} not found.", dll_path);
                0
            }
        }
    }
}

pub fn get_library_handle(emu: &mut emu::Emu, libname: &str) -> u64 {
    let Some(dll) = kernel32_common::normalize_library_name(libname, false) else {
        emu.regs_mut().rax = 0;
        return 0;
    };

    peb64::get_module_base(&dll, emu).unwrap_or(0)
}
