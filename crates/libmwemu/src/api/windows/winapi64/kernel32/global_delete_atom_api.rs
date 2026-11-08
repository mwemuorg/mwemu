use crate::emu;

pub fn GlobalDeleteAtom_api(emu: &mut emu::Emu) {
    let atom = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalDeleteAtom atom:0x{:x}", atom);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
