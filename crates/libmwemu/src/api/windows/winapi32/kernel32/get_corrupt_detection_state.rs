use crate::emu;
use crate::windows::constants;

pub fn GetCorruptDetectionState(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCorruptDetectionState");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
