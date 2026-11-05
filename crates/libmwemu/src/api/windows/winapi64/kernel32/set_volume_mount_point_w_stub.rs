use crate::emu;

pub fn SetVolumeMountPointWStub(emu: &mut emu::Emu) {
    let volume_mount_point = emu.regs().rcx;
    let volume_name = emu.regs().rdx;
    let mount_point = emu.maps.read_wide_string(volume_mount_point);
    let vol_name = emu.maps.read_wide_string(volume_name);
    log_red!(
        emu,
        "kernel32!SetVolumeMountPointWStub mount:'{}' vol:'{}'",
        mount_point,
        vol_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
