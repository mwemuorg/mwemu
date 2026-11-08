use crate::emu;
use crate::windows::constants;

pub fn WerpRecoveryInvokedRemotely(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!WerpRecoveryInvokedRemotely");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
