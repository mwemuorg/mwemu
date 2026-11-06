use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub fn ApplicationRecoveryFinished(emu: &mut emu::Emu) {
    let _bSuccess = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!ApplicationRecoveryFinished");
    helper::advance_tick(emu, 1);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
