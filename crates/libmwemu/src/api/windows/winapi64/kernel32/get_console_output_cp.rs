use crate::emu;

pub fn GetConsoleOutputCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleOutputCP => 437 (OEM US)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 437;
}
