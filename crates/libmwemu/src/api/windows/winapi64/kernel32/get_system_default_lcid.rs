use crate::emu;

pub fn GetSystemDefaultLCID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetSystemDefaultLCID => 0x0409 (en-US)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x0409;
}
