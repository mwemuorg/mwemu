use crate::emu;
use crate::windows::constants;

pub fn MachinePrefersNumericFirstYear(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!MachinePrefersNumericFirstYear");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
