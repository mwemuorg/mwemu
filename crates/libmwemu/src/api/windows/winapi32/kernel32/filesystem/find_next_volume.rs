use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn FindNextVolumeA(emu: &mut emu::Emu) {
    let _hFindVolume = WinApiAbi::arg(emu, 0) as u32;
    let lpszVolumeName = WinApiAbi::arg(emu, 1) as u32;
    let _cchBufferLength = WinApiAbi::arg(emu, 2) as u32;

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!FindNextVolumeA {}", lpszVolumeName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn FindNextVolumeW(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!FindNextVolumeW '{}' p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
