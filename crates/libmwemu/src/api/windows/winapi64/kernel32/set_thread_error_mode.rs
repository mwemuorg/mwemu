use crate::emu;

pub fn SetThreadErrorMode(emu: &mut emu::Emu) {
    let new_mode = emu.regs().rcx as u32;
    let p_old_mode = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!SetThreadErrorMode mode:0x{:x} pOld:0x{:x}",
        new_mode,
        p_old_mode
    );
    if p_old_mode != 0 {
        emu.maps.write_dword(p_old_mode, 0);
    }
    emu.regs_mut().rax = 1;
}
