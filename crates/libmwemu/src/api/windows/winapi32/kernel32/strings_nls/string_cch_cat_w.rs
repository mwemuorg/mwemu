use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn StringCchCatW(emu: &mut emu::Emu) {
    let _pszDest = WinApiAbi::arg(emu, 0) as u32;
    let _cchDest = WinApiAbi::arg(emu, 1) as u32;
    let _pszSrc = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!StringCchCatW");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
