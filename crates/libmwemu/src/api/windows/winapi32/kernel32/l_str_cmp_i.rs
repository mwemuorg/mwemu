use crate::emu;

pub fn LStrCmpI(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LStrCmpI");
    emu.regs_mut().rax = 0;
}
