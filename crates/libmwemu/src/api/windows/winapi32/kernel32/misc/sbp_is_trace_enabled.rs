use crate::emu;
use crate::windows::constants::errors::*;

pub fn SbpIsTraceEnabled(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!SbpIsTraceEnabled");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
