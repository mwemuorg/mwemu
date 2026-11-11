use crate::emu;
use crate::windows::constants::errors::*;

pub fn CalibrateTimer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CalibrateTimer");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
