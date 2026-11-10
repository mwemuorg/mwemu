use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetCurrentJaEraIndex(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentJaEraIndex");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
