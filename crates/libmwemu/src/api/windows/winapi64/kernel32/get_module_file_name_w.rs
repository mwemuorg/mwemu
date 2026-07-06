use crate::{emu, winapi::winapi64::kernel32::set_last_error, windows::constants};

pub fn GetModuleFileNameW(emu: &mut emu::Emu) {
    let module_handle = emu.regs().rcx;
    let lp_filename = emu.regs().rdx;
    let n_size = emu.regs().r8 as u32;

    log_red!(
        emu,
        "kernel32!GetModuleFileNameW hModule: 0x{:x} lpFilename: 0x{:x} nSize: {}",
        module_handle,
        lp_filename,
        n_size
    );

    // Handle zero size buffer
    if n_size == 0 {
        log_red!(emu, "GetModuleFileNameW: Zero size buffer");
        emu.regs_mut().rax = 0;
        return;
    }

    // Validate buffer pointer
    if lp_filename == 0 || !emu.maps.is_mapped(lp_filename) {
        log_red!(emu, "GetModuleFileNameW: Invalid buffer pointer");
        emu.regs_mut().rax = 0;
        return;
    }

    // Determine which module name to use based on handle
    let module_name = if module_handle == 0 {
        // NULL handle means current process executable
        emu.cfg.module_name.clone()
    } else {
        // TODO: Look up actual module by handle
        // For now, just use the default module name
        emu.cfg.module_name.clone()
    };

    let name_chars = module_name.chars().count();
    let required_chars = name_chars + 1; // +1 for null terminator

    if (n_size as usize) < required_chars {
        // Buffer too small - truncate to fit
        let max_chars = (n_size as usize) - 1; // Reserve space for null terminator
        let truncated: String = module_name.chars().take(max_chars).collect();

        emu.maps.write_wide_string(lp_filename, &truncated);

        log_red!(
            emu,
            "GetModuleFileNameW: Buffer too small, truncated to '{}'",
            truncated
        );

        // Set last error for Windows XP+ behavior
        set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = n_size as u64; // Return buffer size when truncated
    } else {
        // Buffer is large enough
        emu.maps.write_wide_string(lp_filename, &module_name);

        log_red!(
            emu,
            "GetModuleFileNameW: Returning '{}' (length: {})",
            module_name,
            name_chars
        );

        emu.regs_mut().rax = name_chars as u64; // Return actual length (without null terminator)
    }
}
