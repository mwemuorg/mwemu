
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetVolumePathNamesForVolumeNameA(emu: &mut emu::Emu) {
    let lpszVolumeName = emu.regs().rcx as u32;
    let lpszVolumePathNames = emu.regs().rdx as u32;
    let _cchBufferLength = emu.regs().r8 as u32;
    let _lpcchReturnLength = emu.regs().r9 as u32;

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);
    let lpszVolumePathNames = emu.maps.read_string(lpszVolumePathNames as u64);

    log_red!(
        emu,
        "kernel32!GetVolumePathNamesForVolumeNameA {} {}",
        lpszVolumeName,
        lpszVolumePathNames
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
