use crate::emu;

pub fn GetConsoleCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleCP => 437 (OEM US)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 437;
}
