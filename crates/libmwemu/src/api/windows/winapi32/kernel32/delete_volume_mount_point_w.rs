use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DeleteVolumeMountPointW(emu: &mut emu::Emu) {
    let mount_point_ptr = arg(emu, 0) as u64;
    let mount_point = emu.maps.read_wide_string(mount_point_ptr);
    log_red!(emu, "kernel32!DeleteVolumeMountPointW '{}'", mount_point);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 1);
}
