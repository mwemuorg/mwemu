use crate::emu;

pub fn AreFileApisANSI(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!AreFileApisANSI");
    emu.regs_mut().rax = 1;
}
