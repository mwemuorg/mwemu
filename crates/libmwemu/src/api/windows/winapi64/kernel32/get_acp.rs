use crate::emu;

pub fn GetACP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetACP => 1252 (Windows-1252)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1252;
}
