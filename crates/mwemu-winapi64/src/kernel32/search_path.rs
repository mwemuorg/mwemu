use crate::emu;

pub fn SearchPathA(emu: &mut emu::Emu) {
    let _path = emu.regs().rcx;
    let filename = emu.regs().rdx;
    let _ext = emu.regs().r8;
    let name_s = if filename != 0 {
        emu.maps.read_string(filename)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!SearchPathA '{}' => 0 (not found)", name_s);
    emu.set_last_error(2); // ERROR_FILE_NOT_FOUND
    emu.regs_mut().rax = 0;
}

pub fn SearchPathW(emu: &mut emu::Emu) {
    let _path = emu.regs().rcx;
    let filename = emu.regs().rdx;
    let _ext = emu.regs().r8;
    let name_s = if filename != 0 {
        emu.maps.read_wide_string(filename)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!SearchPathW '{}' => 0 (not found)", name_s);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}
