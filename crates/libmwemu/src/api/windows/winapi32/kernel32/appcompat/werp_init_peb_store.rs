use crate::emu;
use crate::windows::constants::errors::*;

pub fn WerpInitPEBStore(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpInitPEBStore");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
