use crate::emu;

pub fn GetFileAttributesA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_string(path_ptr);
    log_red!(emu, "kernelbase!GetFileAttributesA '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80;
}

pub fn GetFileAttributesW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_wide_string(path_ptr);
    log_red!(emu, "kernelbase!GetFileAttributesW '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80;
}
