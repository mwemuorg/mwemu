use crate::{emu, winapi::winapi64::kernel32::set_last_error, windows::constants};

pub fn GetCurrentDirectoryA(emu: &mut emu::Emu) {
    let buff_len = emu.regs().rcx as u32;
    let buff_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetCurrentDirectoryA nBufferLength: {} lpBuffer: 0x{:x}",
        buff_len,
        buff_ptr
    );

    let current_dir = emu.cfg.cwd_path.clone();
    let dir_byte_count = current_dir.len(); // Use len() for byte count in ANSI strings

    // When buffer length is 0 or buffer is null, return required size INCLUDING null terminator
    if buff_len == 0 || buff_ptr == 0 {
        set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = (dir_byte_count + 1) as u64; // +1 for null terminator
        return;
    }

    // Check if buffer is large enough (need space for string + null terminator)
    if (buff_len as usize) < (dir_byte_count + 1) {
        set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        // Return required size INCLUDING null terminator
        emu.regs_mut().rax = (dir_byte_count + 1) as u64;
        return;
    }

    // Buffer is large enough, write the directory
    emu.maps.write_string(buff_ptr, &current_dir);

    log_red!(
        emu,
        "GetCurrentDirectoryA returning: '{}' (length: {})",
        current_dir,
        dir_byte_count
    );

    // Return number of characters written (NOT including null terminator)
    emu.regs_mut().rax = dir_byte_count as u64;
    set_last_error(0);
}
