use crate::emu;
use crate::windows::constants;

pub fn GetVolumeNameForVolumeMountPointA(emu: &mut emu::Emu) {
    let lpszFileName = emu.regs().rcx as u32;
    let lpszVolumePathName = emu.regs().rdx as u32;
    let _cchBufferLength = emu.regs().r8 as u32;

    let lpszFileName = emu.maps.read_string(lpszFileName as u64);
    let lpszVolumePathName = emu.maps.read_string(lpszVolumePathName as u64);

    log_red!(
        emu,
        "kernel32!GetVolumeNameForVolumeMountPointA {} {}",
        lpszFileName,
        lpszVolumePathName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
