use crate::emu;
use crate::windows::constants::errors::*;

pub fn CreateSocketHandle(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateSocketHandle");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
