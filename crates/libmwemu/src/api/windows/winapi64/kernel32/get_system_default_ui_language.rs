use crate::emu;

pub fn GetSystemDefaultUILanguage(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetSystemDefaultUILanguage => 0x0409 (en-US)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x0409;
}
