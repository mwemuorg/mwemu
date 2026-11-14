use crate::emu;
use crate::windows::constants::errors::*;

pub fn LZStart(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LZStart");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
