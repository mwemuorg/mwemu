use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn FindFirstVolumeA(emu: &mut emu::Emu) {
    let lpszVolumeName = WinApiAbi::arg(emu, 0) as u32;
    let _cchBufferLength = WinApiAbi::arg(emu, 1) as u32;

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!FindFirstVolumeA {}", lpszVolumeName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn FindFirstVolumeW(emu: &mut emu::Emu) {
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!FindFirstVolumeW");
    emu.regs_mut().rax = helper::handler_create("FindFirstVolumeW");
}
