use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn GetDurationFormat(emu: &mut emu::Emu) {
    let _Locale = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpDuration = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _ullDuration = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpFormat = WinApiAbi::arg(emu, 5) as u32;
    let _lpDurationStr = WinApiAbi::arg(emu, 6) as u32;
    let _cchDuration = WinApiAbi::arg(emu, 7) as u32;

    log_red!(emu, "kernel32!GetDurationFormat");

    emu.regs_mut().rax = ERROR_SUCCESS;

    stdcall_ret(emu, 8);
}
