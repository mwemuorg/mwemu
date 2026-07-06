use crate::{emu, winapi::winapi64::kernel32::set_last_error, windows::constants};

pub fn GetCurrentDirectoryW(emu: &mut emu::Emu) {
    let buff_len = emu.regs().rcx as u32;
    let buff_ptr = emu.regs().rdx;

    let current_dir = emu.cfg.cwd_path.clone();
    let dir_char_count = current_dir.chars().count(); // Use chars().count() for proper Unicode handling

    log_red!(
        emu,
        "kernel32!GetCurrentDirectoryW nBufferLength: {} lpBuffer: 0x{:x}",
        buff_len,
        buff_ptr,
    );

    // When buffer length is 0 or buffer is null, return required size INCLUDING null terminator
    if buff_len == 0 || buff_ptr == 0 {
        set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = (dir_char_count + 2) as u64; // real api would return +1
        return;
    }

    // Check if buffer is large enough (need space for string + null terminator)
    if (buff_len as usize) < (dir_char_count + 1) {
        set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        // Return required size INCLUDING null terminator
        emu.regs_mut().rax = dir_char_count as u64;
        return;
    }

    // Buffer is large enough, write the directory
    emu.maps.write_wide_string(buff_ptr, &current_dir);

    log_red!(
        emu,
        "GetCurrentDirectoryW returning: '{}' (length: {})",
        current_dir,
        dir_char_count
    );

    // Return number of characters written (NOT including null terminator)
    set_last_error(0);
    emu.regs_mut().rax = dir_char_count as u64;
}
