use crate::emu;

pub fn SetVolumeLabelW(emu: &mut emu::Emu) {
    let lpRootPathName = emu.regs().rcx as u32;
    let lpVolumeName = emu.regs().rdx as u32;

    let lpRootPathName = emu.maps.read_wide_string(lpRootPathName as u64);
    let lpVolumeName = emu.maps.read_wide_string(lpVolumeName as u64);

    log_red!(
        emu,
        "kernel32!SetVolumeLabelW {} {}",
        lpRootPathName,
        lpVolumeName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
