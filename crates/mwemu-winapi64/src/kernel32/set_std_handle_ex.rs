use crate::emu;

pub fn SetStdHandleEx(emu: &mut emu::Emu) {
    let std_handle = emu.regs().rcx as u32;
    let handle = emu.regs().rdx;
    let p_prev_value = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!SetStdHandleEx nStdHandle:{} handle:0x{:x} pPrev:0x{:x}",
        std_handle,
        handle,
        p_prev_value
    );
    if p_prev_value != 0 {
        emu.maps.write_qword(p_prev_value, 0);
    }
    emu.regs_mut().rax = 1;
}
