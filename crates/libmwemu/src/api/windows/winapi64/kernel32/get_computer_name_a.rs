use crate::{emu, winapi::winapi64::kernel32::set_last_error, windows::constants};

pub fn GetComputerNameA(emu: &mut emu::Emu) {
    let buff_ptr = emu.regs().rcx; // LPSTR lpBuffer
    let size_ptr = emu.regs().rdx; // LPDWORD nSize

    log_red!(
        emu,
        "kernel32!GetComputerNameA lpBuffer: 0x{:x} nSize: 0x{:x}",
        buff_ptr,
        size_ptr
    );

    // Check if size pointer is valid
    if size_ptr == 0 || !emu.maps.is_mapped(size_ptr) {
        log_red!(emu, "GetComputerNameA: Invalid nSize pointer");
        set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read current buffer size (in bytes)
    let buffer_size = emu
        .maps
        .read_dword(size_ptr)
        .expect("Cannot read buffer size") as usize;

    // Calculate required size in bytes
    let host_name = emu.cfg.host_name.clone();
    let computer_name_bytes = host_name.len();
    let required_size_with_null = computer_name_bytes + 1; // +1 for null terminator

    // Check if output buffer is valid (only if buffer_size > 0)
    if buffer_size > 0 && (buff_ptr == 0 || !emu.maps.is_mapped(buff_ptr)) {
        log_red!(emu, "GetComputerNameA: Invalid lpBuffer pointer");
        set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Check if buffer is large enough
    if buffer_size < required_size_with_null {
        log_red!(
            emu,
            "GetComputerNameA: Buffer too small. Required: {}, Provided: {}",
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
    emu.maps.write_string(buff_ptr, &host_name);

    // On success, write the number of bytes copied (NOT including null terminator)
    emu.maps.write_dword(size_ptr, computer_name_bytes as u32);

    log_red!(
        emu,
        "kernel32!GetComputerNameA returning: '{}' (bytes: {})",
        host_name,
        computer_name_bytes
    );

    emu.regs_mut().rax = constants::TRUE;
}
