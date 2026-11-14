use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetCompatFlags(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCompatFlags");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
