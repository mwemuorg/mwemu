use crate::emu;

pub fn GetConsoleCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleCP");
    emu.regs_mut().rax = 0x00000409;
}
