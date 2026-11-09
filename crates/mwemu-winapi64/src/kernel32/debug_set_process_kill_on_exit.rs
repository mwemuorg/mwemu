
use crate::windows::constants::errors::*;
use crate::emu;
use crate::winapi::helper;

pub fn DebugSetProcessKillOnExit(emu: &mut emu::Emu) {
    let _KillOnExit = emu.regs().rcx as u32;

    log_red!(emu, "kernel32!DebugSetProcessKillOnExit");
    helper::advance_tick(emu, 1);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
