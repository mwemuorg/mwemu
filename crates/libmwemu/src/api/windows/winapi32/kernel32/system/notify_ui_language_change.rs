use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn NotifyUILanguageChange(emu: &mut emu::Emu) {
    let _dwFlags = WinApiAbi::arg(emu, 0) as u32;
    let _pcwstrNewLanguage = WinApiAbi::arg(emu, 1) as u32;
    let _pcwstrPreviousLanguage = WinApiAbi::arg(emu, 2) as u32;
    let _dwReserved = WinApiAbi::arg(emu, 3) as u32;
    let _pdwStatusRtrn = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!NotifyUILanguageChange");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
