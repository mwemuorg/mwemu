use crate::emu;
use crate::windows::constants::errors::*;

pub fn Wow64TransitionResolve(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!Wow64TransitionResolve");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
