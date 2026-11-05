use crate::emu;

pub fn GetActiveProcessorGroupCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetActiveProcessorGroupCount => 1");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
