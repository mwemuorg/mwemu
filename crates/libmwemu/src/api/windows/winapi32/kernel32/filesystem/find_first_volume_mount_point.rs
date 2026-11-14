use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn FindFirstVolumeMountPointA(emu: &mut emu::Emu) {
    find_first_volume_mount_point(emu, "FindFirstVolumeMountPointA", false);
}

fn find_first_volume_mount_point(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpszRootPathName = WinApiAbi::arg(emu, 0) as u32;
    let _lpszVolumeMountPoint = WinApiAbi::arg(emu, 1) as u32;
    let _cchBufferLength = WinApiAbi::arg(emu, 2) as u32;

    let lpszRootPathName = strings::read_text(emu, lpszRootPathName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpszRootPathName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn FindFirstVolumeMountPointW(emu: &mut emu::Emu) {
    find_first_volume_mount_point(emu, "FindFirstVolumeMountPointW", true);
}
