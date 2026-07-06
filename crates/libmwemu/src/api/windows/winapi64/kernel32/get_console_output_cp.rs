use crate::emu;

pub fn GetConsoleOutputCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleOutputCP");
    emu.regs_mut().rax = 0x00000409;
}
