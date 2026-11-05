use crate::emu;

pub fn InitAtomTable(emu: &mut emu::Emu) {
    let n_size = emu.regs().rcx;
    log_red!(emu, "kernel32!InitAtomTable size:{}", n_size);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
