use crate::emu;

pub fn SetCurrentDirectoryA(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let path = emu.maps.read_string(path_ptr);

    log_red!(emu, "kernel32!SetCurrentDirectoryA '{}'", path);

    if !path.is_empty() {
        emu.cfg.cwd_path = path;
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
