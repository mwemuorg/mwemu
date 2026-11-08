use crate::emu;
use crate::windows::constants;

pub fn BasepInitializeTermsrvFpns(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepInitializeTermsrvFpns");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
