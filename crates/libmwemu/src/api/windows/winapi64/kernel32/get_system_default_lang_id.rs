use crate::emu;

pub fn GetSystemDefaultLangID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetSystemDefaultLangID => 0x0409 (en-US)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x0409;
}
