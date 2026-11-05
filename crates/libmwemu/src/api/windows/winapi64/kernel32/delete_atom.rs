use crate::emu;

pub fn DeleteAtom(emu: &mut emu::Emu) {
    let atom = emu.regs().rcx as u16;
    log_red!(emu, "kernel32!DeleteAtom atom:{}", atom);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // success = 0
}
