use crate::emu;
use crate::winapi::common::strings;

pub fn GetDriveTypeA(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeA", false);
}

fn get_drive_type(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path = strings::read_text(emu, emu.regs().rcx, wide);
    log_red!(emu, "kernel32!{api} '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 3; // DRIVE_FIXED
}

pub fn GetDriveTypeW(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeW", true);
}
