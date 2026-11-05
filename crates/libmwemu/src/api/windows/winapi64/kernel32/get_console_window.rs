use crate::emu;

pub fn GetConsoleWindow(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleWindow => 0x00010020");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x00010020;
}
