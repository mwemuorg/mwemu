use crate::{emu, windows::structures};

pub fn GetVersionExA(emu: &mut emu::Emu) {
    let version_info_ptr = emu.regs().rcx;

    log_red!(emu, "kernel32!GetVersionExA 0x{:x}", version_info_ptr);

    // Check if pointer is valid
    if version_info_ptr == 0 || !emu.maps.is_mapped(version_info_ptr) {
        log_red!(emu, "GetVersionExA: Invalid pointer");
        emu.regs_mut().rax = 0;
        return;
    }

    // Read the dwOSVersionInfoSize field (first 4 bytes) to determine structure type
    let struct_size = emu
        .maps
        .read_dword(version_info_ptr)
        .expect("Cannot read struct size");

    log_red!(emu, "GetVersionExA: Structure size: {}", struct_size);

    // Determine which structure type based on size
    const OSVERSIONINFOA_SIZE: u32 = 148; // Basic structure
    const OSVERSIONINFOEXA_SIZE: u32 = 284; // Extended structure

    let use_extended = match struct_size {
        OSVERSIONINFOA_SIZE => {
            log_red!(emu, "Using OSVERSIONINFOA (basic)");
            false
        }
        OSVERSIONINFOEXA_SIZE => {
            log_red!(emu, "Using OSVERSIONINFOEXA (extended)");
            true
        }
        _ => {
            log_red!(
                emu,
                "GetVersionExA: Invalid struct size: {} (expected {} or {})",
                struct_size,
                OSVERSIONINFOA_SIZE,
                OSVERSIONINFOEXA_SIZE
            );
            emu.regs_mut().rax = 0;
            return;
        }
    };

    if use_extended {
        let os_version_info_ex_a = structures::OsVersionInfoExA::new();
        os_version_info_ex_a.save(version_info_ptr, &mut emu.maps);
    } else {
        let os_version_info_a = structures::OsVersionInfoA::new();
        os_version_info_a.save(version_info_ptr, &mut emu.maps);
    }

    emu.regs_mut().rax = 1;
}
