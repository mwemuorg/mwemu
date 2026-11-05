use crate::emu;

pub fn GetDriveTypeW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(emu.regs().rcx);
    log_red!(emu, "kernel32!GetDriveTypeW '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 3; // DRIVE_FIXED
}
