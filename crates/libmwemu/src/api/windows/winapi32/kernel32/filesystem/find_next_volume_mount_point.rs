use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn FindNextVolumeMountPointA(emu: &mut emu::Emu) {
    find_next_volume_mount_point(emu, "FindNextVolumeMountPointA");
}

fn find_next_volume_mount_point(emu: &mut emu::Emu, api: &str) {
    let _hFindVolumeMountPoint = WinApiAbi::arg(emu, 0) as u32;
    let _lpszVolumeMountPoint = WinApiAbi::arg(emu, 1) as u32;
    let _cchBufferLength = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn FindNextVolumeMountPointW(emu: &mut emu::Emu) {
    find_next_volume_mount_point(emu, "FindNextVolumeMountPointW");
}
