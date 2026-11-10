use crate::emu;
use crate::windows::constants::errors::*;

pub fn WerpAcquirePebLock(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpAcquirePebLock");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
