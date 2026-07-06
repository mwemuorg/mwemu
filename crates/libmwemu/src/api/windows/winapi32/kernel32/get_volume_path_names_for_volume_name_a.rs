use crate::emu;
use crate::windows::constants;

pub fn GetVolumePathNamesForVolumeNameA(emu: &mut emu::Emu) {
    let lpszVolumeName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpszVolumePathNames = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _cchBufferLength = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _lpcchReturnLength = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);
    let lpszVolumePathNames = emu.maps.read_string(lpszVolumePathNames as u64);

    log_red!(
        emu,
        "kernel32!GetVolumePathNamesForVolumeNameA {} {}",
        lpszVolumeName,
        lpszVolumePathNames
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
