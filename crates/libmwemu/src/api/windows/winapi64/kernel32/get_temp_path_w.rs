use crate::emu;

pub fn GetTempPathW(emu: &mut emu::Emu) {
    /*
    DWORD GetTempPathW(
        [in]  DWORD  nBufferLength,
        [out] LPWSTR lpBuffer
    );
    */
    let n_buffer_length = emu.regs().rcx as u32;
    let lp_buffer = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetTempPathW buffer_len: {} buffer: 0x{:x}",
        n_buffer_length,
        lp_buffer
    );

    let temp_path = emu.cfg.temp_path.clone();
    let required_length = temp_path.len() as u32 + 1; // +1 for null terminator

    // If buffer length is 0 or buffer is null, return required length
    if n_buffer_length == 0 || lp_buffer == 0 {
        emu.regs_mut().rax = required_length as u64;
        return;
    }

    // Check if buffer is large enough
    if n_buffer_length < required_length {
        // Buffer too small, return required length
        emu.regs_mut().rax = required_length as u64;
        return;
    }

    // Write the temp path to the buffer
    emu.maps.write_wide_string(lp_buffer, &temp_path);

    // Return the number of characters copied (excluding null terminator)
    emu.regs_mut().rax = (required_length - 1) as u64;
}
