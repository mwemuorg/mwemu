use crate::emu;
use crate::windows::constants;

pub fn GetVolumeNameForVolumeMountPointA(emu: &mut emu::Emu) {
    let lpszFileName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpszVolumePathName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _cchBufferLength = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpszFileName = emu.maps.read_string(lpszFileName as u64);
    let lpszVolumePathName = emu.maps.read_string(lpszVolumePathName as u64);

    log_red!(
        emu,
        "kernel32!GetVolumeNameForVolumeMountPointA {} {}",
        lpszFileName,
        lpszVolumePathName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
