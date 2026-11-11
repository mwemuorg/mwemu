use crate::emu;
use crate::windows::constants::errors::*;

pub fn timeGetTime(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!timeGetTime");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
