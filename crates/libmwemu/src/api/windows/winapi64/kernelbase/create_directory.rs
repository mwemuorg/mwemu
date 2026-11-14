use crate::emu;
use crate::winapi::common::strings;

pub fn CreateDirectoryA(emu: &mut emu::Emu) {
    create_directory(emu, "CreateDirectoryA", false);
}

fn create_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let security = emu.regs().rdx;
    let path = strings::read_text(emu, path_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CreateDirectoryW(emu: &mut emu::Emu) {
    create_directory(emu, "CreateDirectoryW", true);
}
