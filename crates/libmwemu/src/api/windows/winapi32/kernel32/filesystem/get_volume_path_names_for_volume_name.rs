use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn GetVolumePathNamesForVolumeNameA(emu: &mut emu::Emu) {
    let lpszVolumeName = WinApiAbi::arg(emu, 0) as u32;
    let lpszVolumePathNames = WinApiAbi::arg(emu, 1) as u32;
    let _cchBufferLength = WinApiAbi::arg(emu, 2) as u32;
    let _lpcchReturnLength = WinApiAbi::arg(emu, 3) as u32;

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);
    let lpszVolumePathNames = emu.maps.read_string(lpszVolumePathNames as u64);

    log_red!(
        emu,
        "kernel32!GetVolumePathNamesForVolumeNameA {} {}",
        lpszVolumeName,
        lpszVolumePathNames
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

pub fn GetVolumePathNamesForVolumeNameW(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(
        emu,
        "kernel32!GetVolumePathNamesForVolumeNameW '{}'  0x{:x}",
        s,
        arg2
    );
    stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}
