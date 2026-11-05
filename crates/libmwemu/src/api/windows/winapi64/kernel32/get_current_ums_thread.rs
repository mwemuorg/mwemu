use crate::emu;

pub fn GetCurrentUmsThread(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentUmsThread => 0 (UMS not available)");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
