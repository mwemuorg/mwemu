use crate::emu;

pub fn SetVolumeMountPointW(emu: &mut emu::Emu) {
    let _lpszVolumeMountPoint = emu.regs().rcx as u32;
    let lpszVolumeName = emu.regs().rdx as u32;

    let lpszVolumeName = emu.maps.read_wide_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!SetVolumeMountPointW {}", lpszVolumeName);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
