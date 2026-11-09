use crate::emu;
use crate::windows::constants::errors::*;

pub fn BasepInitializeTermsrvFpns(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepInitializeTermsrvFpns");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
