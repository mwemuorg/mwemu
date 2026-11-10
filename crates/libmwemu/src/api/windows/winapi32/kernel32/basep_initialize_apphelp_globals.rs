use crate::emu;
use crate::windows::constants::errors::*;

pub fn BasepInitializeApphelpGlobals(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepInitializeApphelpGlobals");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
