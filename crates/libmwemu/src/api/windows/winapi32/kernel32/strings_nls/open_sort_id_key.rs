use crate::emu;
use crate::windows::constants::errors::*;

pub fn OpenSortIdKey(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenSortIdKey");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
