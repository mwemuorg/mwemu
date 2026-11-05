use crate::emu;

pub fn GetThreadErrorMode(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetThreadErrorMode => 0 (default)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
