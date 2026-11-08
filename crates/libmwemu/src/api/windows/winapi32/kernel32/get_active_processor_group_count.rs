use crate::emu;
use crate::windows::constants;

pub fn GetActiveProcessorGroupCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetActiveProcessorGroupCount");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
