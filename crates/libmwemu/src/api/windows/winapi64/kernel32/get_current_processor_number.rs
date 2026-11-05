use crate::emu;

pub fn GetCurrentProcessorNumber(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentProcessorNumber => 0");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
