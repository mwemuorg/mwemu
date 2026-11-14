use crate::emu;
use crate::winapi::common::strings;

pub fn GetDriveTypeA(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeA", false);
}

fn get_drive_type(emu: &mut emu::Emu, api: &str, wide: bool) {
    let root_ptr = emu.regs().rcx;
    let root = if root_ptr != 0 {
        strings::read_text(emu, root_ptr, wide)
    } else {
        String::from("C:\\")
    };
    log_red!(emu, "kernelbase!{api} '{}' => DRIVE_FIXED", root);
    emu.set_last_error(0);
    emu.regs_mut().rax = 3;
}

pub fn GetDriveTypeW(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeW", true);
}
