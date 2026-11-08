use crate::emu;
use crate::windows::constants;

pub fn GetMaximumProcessorGroupCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetMaximumProcessorGroupCount");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
