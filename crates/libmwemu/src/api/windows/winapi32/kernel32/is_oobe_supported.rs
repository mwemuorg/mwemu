use crate::emu;
use crate::windows::constants::errors::*;

pub fn IsOOBESupported(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!IsOOBESupported");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
