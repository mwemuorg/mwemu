use crate::emu;
use crate::windows::constants;

pub fn IsFusionFullySupported(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsFusionFullySupported");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
