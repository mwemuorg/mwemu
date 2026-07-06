use crate::{emu, winapi::winapi64::kernel32::set_last_error, windows::constants};

pub fn GetComputerNameW(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx; // LPWSTR lpBuffer
    let size_ptr = emu.regs().rdx; // LPDWORD nSize

    log_red!(
        emu,
        "kernel32!GetComputerNameW lpBuffer: 0x{:x} nSize: 0x{:x}",
        buff_ptr,
        size_ptr
    );

    // Check if size pointer is valid
    if size_ptr == 0 || !emu.maps.is_mapped(size_ptr) {
        log_red!(emu, "GetComputerNameW: Invalid nSize pointer");
        set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read current buffer size (in characters)
    let buffer_size = emu
        .maps
        .read_dword(size_ptr)
        .expect("Cannot read buffer size") as usize;

    // Calculate required size in characters
    let host_name = emu.cfg.host_name.clone();
    let computer_name_chars = host_name.chars().count();
    let required_size_with_null = computer_name_chars + 1; // +1 for null terminator

    // Check if output buffer is valid (only if buffer_size > 0)
    if buffer_size > 0 && (buff_ptr == 0 || !emu.maps.is_mapped(buff_ptr)) {
        log_red!(emu, "GetComputerNameW: Invalid lpBuffer pointer");
        set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Check if buffer is large enough
    if buffer_size < required_size_with_null {
        log_red!(
            emu,
            "GetComputerNameW: Buffer too small. Required: {}, Provided: {}",
            required_size_with_null,
            buffer_size
        );
        // Set size to required size (including null terminator)
        emu.maps
            .write_dword(size_ptr, required_size_with_null as u32);
        set_last_error(constants::ERROR_BUFFER_OVERFLOW);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Buffer is large enough, write the computer name
    emu.maps.write_wide_string(buff_ptr, &host_name);

    // On success, write the number of characters copied (NOT including null terminator)
    emu.maps.write_dword(size_ptr, computer_name_chars as u32);

    log_red!(
        emu,
        "kernel32!GetComputerNameW returning: '{}' (chars: {})",
        host_name,
        computer_name_chars
    );

    emu.regs_mut().rax = constants::TRUE;
}
