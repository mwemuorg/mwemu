use crate::emu;

pub fn GetTempPathA(emu: &mut emu::Emu) {
    let n_buffer_length = emu.regs().rcx as u32;
    let lp_buffer = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!GetTempPathA buffer_len: {} buffer: 0x{:x}",
        n_buffer_length,
        lp_buffer
    );

    let temp_path = format!("{}\\", emu.cfg.temp_path);
    let required_length = temp_path.len() as u32 + 1;

    if n_buffer_length == 0 || lp_buffer == 0 {
        emu.regs_mut().rax = required_length as u64;
        return;
    }

    if n_buffer_length < required_length {
        emu.regs_mut().rax = required_length as u64;
        return;
    }

    emu.maps.write_string(lp_buffer, &temp_path);
    emu.regs_mut().rax = (required_length - 1) as u64;
}
