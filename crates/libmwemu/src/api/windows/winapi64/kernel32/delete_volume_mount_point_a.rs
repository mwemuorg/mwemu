use crate::emu;

pub fn DeleteVolumeMountPointA(emu: &mut emu::Emu) {
    let mount_point_ptr = emu.regs().rcx;
    let mount_point = emu.maps.read_string(mount_point_ptr);
    log_red!(emu, "kernel32!DeleteVolumeMountPointA '{}'", mount_point);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
