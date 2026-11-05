use crate::emu;
use crate::windows::constants;

pub fn ApplicationRecoveryFinished(emu: &mut emu::Emu) {
    let _bSuccess = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!ApplicationRecoveryFinished");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
