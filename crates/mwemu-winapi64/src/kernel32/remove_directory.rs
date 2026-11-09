use crate::emu;

pub fn RemoveDirectoryA(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rcx);
    log_red!(emu, "kernel32!RemoveDirectoryA '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn RemoveDirectoryW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(emu.regs().rcx);
    log_red!(emu, "kernel32!RemoveDirectoryW '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
