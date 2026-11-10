use crate::emu;
use crate::windows::constants::errors::*;

pub fn WerpCheckOkToRegister(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpCheckOkToRegister");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
