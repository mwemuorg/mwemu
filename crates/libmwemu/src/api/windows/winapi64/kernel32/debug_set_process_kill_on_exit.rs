use crate::emu;
use crate::windows::constants;

pub fn DebugSetProcessKillOnExit(emu: &mut emu::Emu) {
    let _KillOnExit = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!DebugSetProcessKillOnExit");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
