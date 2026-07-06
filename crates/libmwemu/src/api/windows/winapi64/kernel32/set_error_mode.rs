use crate::emu;

pub fn SetErrorMode(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx;

    log_red!(emu, "kernel32!SetErrorMode 0x{:x}", mode);

    emu.regs_mut().rax = 0;
}
