use crate::emu;

pub fn LStrCmpIW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LStrCmpIW");
    emu.regs_mut().rax = 0;
}
