use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn GetDurationFormat(emu: &mut emu::Emu) {
    let _Locale = WinApiAbi::arg(emu, 0) as u32;
    let _dwFlags = WinApiAbi::arg(emu, 1) as u32;
    let _lpDuration = WinApiAbi::arg(emu, 2) as u32;
    let _ullDuration = WinApiAbi::arg(emu, 3) as u32;
    let _lpFormat = WinApiAbi::arg(emu, 5) as u32;
    let _lpDurationStr = WinApiAbi::arg(emu, 6) as u32;
    let _cchDuration = WinApiAbi::arg(emu, 7) as u32;

    log_red!(emu, "kernel32!GetDurationFormat");

    emu.regs_mut().rax = ERROR_SUCCESS;

    stdcall_ret(emu, 8);
}
