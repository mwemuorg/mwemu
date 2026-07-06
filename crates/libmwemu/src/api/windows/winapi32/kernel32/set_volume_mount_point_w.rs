use crate::emu;
use crate::windows::constants;

pub fn SetVolumeMountPointW(emu: &mut emu::Emu) {
    let _lpszVolumeMountPoint = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpszVolumeName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let lpszVolumeName = emu.maps.read_wide_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!SetVolumeMountPointW {}", lpszVolumeName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
