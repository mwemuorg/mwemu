use crate::emu;

pub fn GetDriveTypeA(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rcx);
    log_red!(emu, "kernel32!GetDriveTypeA '{}'", path);
    emu.regs_mut().rax = 3; // DRIVE_FIXED
}
