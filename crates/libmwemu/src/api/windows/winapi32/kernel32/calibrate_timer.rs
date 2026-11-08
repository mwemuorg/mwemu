use crate::emu;
use crate::windows::constants;

pub fn CalibrateTimer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CalibrateTimer");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
