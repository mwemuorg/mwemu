use crate::emu;

pub fn CreateDirectoryW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let _security = emu.regs().rdx;

    let path = emu.maps.read_wide_string(path_ptr);

    log_red!(emu, "kernel32!CreateDirectoryW `{}` (stub)", path);

    emu.regs_mut().rax = 1;
}
