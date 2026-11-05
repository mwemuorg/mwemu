use crate::emu;

pub fn GetMaximumProcessorGroupCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaximumProcessorGroupCount => 1");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
