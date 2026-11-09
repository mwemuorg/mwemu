use crate::emu;

pub fn SetVolumeMountPointA(emu: &mut emu::Emu) {
    let _lpszVolumeMountPoint = emu.regs().rcx as u32;
    let lpszVolumeName = emu.regs().rdx as u32;

    let lpszVolumeName = emu.maps.read_string(lpszVolumeName as u64);

    log_red!(emu, "kernel32!SetVolumeMountPointA {}", lpszVolumeName);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
