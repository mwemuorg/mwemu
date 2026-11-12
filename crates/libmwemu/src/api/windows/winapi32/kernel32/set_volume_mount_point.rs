use crate::emu;
use crate::winapi::common::strings;
use crate::windows::constants::errors::*;

pub fn SetVolumeMountPointA(emu: &mut emu::Emu) {
    set_volume_mount_point(emu, "SetVolumeMountPointA", false);
}

fn set_volume_mount_point(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _lpszVolumeMountPoint = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpszVolumeName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpszVolumeName = strings::read_text(emu, lpszVolumeName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpszVolumeName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn SetVolumeMountPointW(emu: &mut emu::Emu) {
    set_volume_mount_point(emu, "SetVolumeMountPointW", true);
}
