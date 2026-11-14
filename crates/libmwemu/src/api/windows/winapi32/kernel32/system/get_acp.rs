use crate::emu;

pub fn GetACP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetAcp");
    emu.regs_mut().rax = 1252;
}
