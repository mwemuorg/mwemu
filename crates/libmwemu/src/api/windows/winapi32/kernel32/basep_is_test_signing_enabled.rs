use crate::emu;
use crate::windows::constants;

pub fn BasepIsTestSigningEnabled(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!BasepIsTestSigningEnabled");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
