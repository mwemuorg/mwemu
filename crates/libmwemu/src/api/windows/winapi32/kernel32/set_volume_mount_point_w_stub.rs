use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetVolumeMountPointWStub(emu: &mut emu::Emu) {
    let volume_mount_point = arg(emu, 0) as u64;
    let volume_name = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}
