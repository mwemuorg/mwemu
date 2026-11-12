use crate::emu;
use crate::windows::constants::errors::*;

pub fn FindNextVolumeMountPointA(emu: &mut emu::Emu) {
    find_next_volume_mount_point(emu, "FindNextVolumeMountPointA");
}

fn find_next_volume_mount_point(emu: &mut emu::Emu, api: &str) {
    let _hFindVolumeMountPoint = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpszVolumeMountPoint = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _cchBufferLength = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!{api}");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn FindNextVolumeMountPointW(emu: &mut emu::Emu) {
    find_next_volume_mount_point(emu, "FindNextVolumeMountPointW");
}
