use crate::emu;
use crate::windows::constants;

pub fn SetVolumeLabelA(emu: &mut emu::Emu) {
    let lpRootPathName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpVolumeName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let lpRootPathName = emu.maps.read_string(lpRootPathName as u64);
    let lpVolumeName = emu.maps.read_string(lpVolumeName as u64);

    log_red!(
        emu,
        "kernel32!SetVolumeLabelA {} {}",
        lpRootPathName,
        lpVolumeName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
