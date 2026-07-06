use crate::emu;
use crate::windows::constants;

pub fn FindNextVolumeA(emu: &mut emu::Emu) {
    let _hFindVolume = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpszVolumeName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _cchBufferLength = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!FindNextVolumeA {}", lpszVolumeName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
