use crate::emu;

pub fn GlobalFindAtomA(emu: &mut emu::Emu) {
    let name = emu.regs().rcx;
    let name_s = if name != 0 {
        emu.maps.read_string(name)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!GlobalFindAtomA '{}' => 0", name_s);
    emu.regs_mut().rax = 0;
    emu.set_last_error(2);
}

pub fn GlobalFindAtomW(emu: &mut emu::Emu) {
    let name = emu.regs().rcx;
    let name_s = if name != 0 {
        emu.maps.read_wide_string(name)
    } else {
        String::new()
    };
    log_red!(emu, "kernel32!GlobalFindAtomW '{}' => 0", name_s);
    emu.regs_mut().rax = 0;
    emu.set_last_error(2);
}
