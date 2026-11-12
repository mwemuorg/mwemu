use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn FindFirstVolumeMountPointA(emu: &mut emu::Emu) {
    find_first_volume_mount_point(emu, "FindFirstVolumeMountPointA", false);
}

fn find_first_volume_mount_point(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpszRootPathName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpszVolumeMountPoint = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _cchBufferLength = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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
