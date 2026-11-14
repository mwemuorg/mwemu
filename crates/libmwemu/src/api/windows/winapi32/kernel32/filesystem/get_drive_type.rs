use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetDriveTypeA(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeA", false);
}

fn get_drive_type(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = WinApiAbi::arg(emu, 0);
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!{api} '{}'", path);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 3; // DRIVE_FIXED
}

pub fn GetDriveTypeW(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeW", true);
}
