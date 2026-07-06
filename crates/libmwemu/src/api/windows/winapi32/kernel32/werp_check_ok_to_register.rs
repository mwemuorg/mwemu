use crate::emu;
use crate::windows::constants;

pub fn WerpCheckOkToRegister(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpCheckOkToRegister");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
