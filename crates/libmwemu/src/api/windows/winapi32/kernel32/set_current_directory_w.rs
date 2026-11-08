use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetCurrentDirectoryW(emu: &mut emu::Emu) {
    let path_ptr = arg(emu, 0) as u64;
    let path = emu.maps.read_wide_string(path_ptr);

    log_red!(emu, "kernel32!SetCurrentDirectoryW '{}'", path);

    if !path.is_empty() {
        emu.cfg.cwd_path = path;
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
