use crate::emu;
use crate::windows::constants;

pub(super) fn GetUserNameW(emu: &mut emu::Emu) {
    let out_username = emu.regs().rcx; // LPWSTR lpBuffer
    let in_out_sz = emu.regs().rdx; // LPDWORD pcbBuffer

    log_red!(
        emu,
        "advapi32!GetUserNameW lpBuffer: 0x{:x} pcbBuffer: 0x{:x}",
        out_username,
        in_out_sz
    );

    // Check if size pointer is valid
    if in_out_sz == 0 || !emu.maps.is_mapped(in_out_sz) {
        log_red!(emu, "GetUserNameW: Invalid pcbBuffer pointer");
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Read current buffer size (in characters)
    let buffer_size = emu.maps.read_dword(in_out_sz).unwrap_or(0) as usize;

    // Calculate required size in characters (including null terminator)
    let user_name = emu.cfg.user_name.clone();
    let username_chars = user_name.chars().count();
    let required_size = username_chars + 1; // +1 for null terminator

    // Always update the size to show required characters
    emu.maps.write_dword(in_out_sz, required_size as u32);

    // Check if output buffer is valid
    if out_username == 0 || !emu.maps.is_mapped(out_username) {
        log_red!(emu, "GetUserNameW: Invalid lpBuffer pointer");
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Check if buffer is large enough
    if buffer_size < required_size {
        log_red!(
            emu,
            "GetUserNameW: Buffer too small. Required: {}, Provided: {}",
            required_size,
            buffer_size
        );
        emu.regs_mut().rax = constants::FALSE;
        return;
    }

    // Buffer is large enough, write the username
    emu.maps.write_wide_string(out_username, &user_name);

    log_red!(
        emu,
        "GetUserNameW returning: '{}' (size: {})",
        user_name,
        required_size
    );

    emu.regs_mut().rax = constants::TRUE;
}
