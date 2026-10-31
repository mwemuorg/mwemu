use crate::emu;

pub fn RemoveDirectoryW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(emu.regs().rcx);
    log_red!(emu, "kernel32!RemoveDirectoryW '{}'", path);
    emu.regs_mut().rax = 1;
}
