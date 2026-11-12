use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn StringCchCopyExW(emu: &mut emu::Emu) {
    let _pszDest = WinApiAbi::arg(emu, 0) as u32;
    let _cchDest = WinApiAbi::arg(emu, 1) as u32;
    let _pszSrc = WinApiAbi::arg(emu, 2) as u32;
    let _ppszDestEnd = WinApiAbi::arg(emu, 3) as u32;
    let _pcchRemaining = WinApiAbi::arg(emu, 4) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 5) as u32;

    log_red!(emu, "kernel32!StringCchCopyExW");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
