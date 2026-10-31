use crate::emu;

pub fn RemoveDirectoryA(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rcx);
    log_red!(emu, "kernel32!RemoveDirectoryA '{}'", path);
    emu.regs_mut().rax = 1;
}
