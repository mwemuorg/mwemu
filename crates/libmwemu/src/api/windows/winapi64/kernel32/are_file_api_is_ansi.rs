use crate::emu;

pub fn AreFileApiIsAnsi(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!AreFileApiIsAnsi");
    emu.regs_mut().rax = 1;
}
