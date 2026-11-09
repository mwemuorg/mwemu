use crate::emu;

pub fn GetDriveTypeW(emu: &mut emu::Emu) {
    let root_ptr = emu.regs().rcx;
    let root = if root_ptr != 0 {
        emu.maps.read_wide_string(root_ptr)
    } else {
        String::from("C:\\")
    };
    log_red!(emu, "kernelbase!GetDriveTypeW '{}' => DRIVE_FIXED", root);
    emu.set_last_error(0);
    emu.regs_mut().rax = 3;
}
