use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetActiveProcessorGroupCount(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetActiveProcessorGroupCount");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
