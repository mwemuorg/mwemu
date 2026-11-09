use crate::emu;

pub fn GetDriveTypeA(emu: &mut emu::Emu) {
    let root_ptr = emu.regs().rcx;
    let root = if root_ptr != 0 {
        emu.maps.read_string(root_ptr)
    } else {
        String::from("C:\\")
    };
    log_red!(emu, "kernelbase!GetDriveTypeA '{}' => DRIVE_FIXED", root);
    emu.set_last_error(0);
    emu.regs_mut().rax = 3;
}
