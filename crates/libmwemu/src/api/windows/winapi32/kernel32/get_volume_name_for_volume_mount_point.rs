use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetVolumeNameForVolumeMountPointA(emu: &mut emu::Emu) {
    let lpszFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpszVolumePathName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _cchBufferLength = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpszFileName = emu.maps.read_string(lpszFileName as u64);
    let lpszVolumePathName = emu.maps.read_string(lpszVolumePathName as u64);

    log_red!(
        emu,
        "kernel32!GetVolumeNameForVolumeMountPointA {} {}",
        lpszFileName,
        lpszVolumePathName
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn GetVolumeNameForVolumeMountPointW(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!GetVolumeNameForVolumeMountPointW '{}' p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
