use crate::emu;
use crate::windows::constants::errors::*;

pub fn LoadAppInitDllsImplementation(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LoadAppInitDllsImplementation");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
