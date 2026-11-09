use crate::emu;

pub fn CreateDirectoryExW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let security = emu.regs().rdx;
    let path = emu.maps.read_wide_string(path_ptr);
    log_red!(emu, "kernelbase!CreateDirectoryExW '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
