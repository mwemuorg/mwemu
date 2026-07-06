use crate::emu;
use crate::windows::constants;

pub fn FindFirstVolumeA(emu: &mut emu::Emu) {
    let lpszVolumeName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _cchBufferLength = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!FindFirstVolumeA {}", lpszVolumeName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
