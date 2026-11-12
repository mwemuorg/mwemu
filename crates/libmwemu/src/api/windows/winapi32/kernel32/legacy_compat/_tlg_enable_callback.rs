use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn _tlgEnableCallback(emu: &mut emu::Emu) {
    let _pSourceId = WinApiAbi::arg(emu, 0) as u32;
    let _callbackType = WinApiAbi::arg(emu, 1) as u32;
    let _level = WinApiAbi::arg(emu, 2) as u32;
    let _keywordAny = WinApiAbi::arg(emu, 3) as u32;
    let _keywordAll = WinApiAbi::arg(emu, 4) as u32;
    let _pFilterData = WinApiAbi::arg(emu, 5) as u32;
    let _pCallbackContext = WinApiAbi::arg(emu, 6) as u32;

    log_red!(emu, "kernel32!_tlgEnableCallback");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
