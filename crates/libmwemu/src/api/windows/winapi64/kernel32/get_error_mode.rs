use crate::emu;

pub fn GetErrorMode(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetErrorMode => 0 (default)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
