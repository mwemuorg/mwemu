use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn RegisterApplicationRecoveryCallback(emu: &mut emu::Emu) {
    let _pRecoveyCallback = WinApiAbi::arg(emu, 0) as u32;
    let _pvParameter = WinApiAbi::arg(emu, 1) as u32;
    let _dwPingInterval = WinApiAbi::arg(emu, 2) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!RegisterApplicationRecoveryCallback");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
