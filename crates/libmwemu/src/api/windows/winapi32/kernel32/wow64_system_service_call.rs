use crate::emu;
use crate::windows::constants::errors::*;

pub fn Wow64SystemServiceCall(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!Wow64SystemServiceCall");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
