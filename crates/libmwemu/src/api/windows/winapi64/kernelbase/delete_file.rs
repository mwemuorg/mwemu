use crate::emu;
use crate::winapi::common::strings;

pub fn DeleteFileA(emu: &mut emu::Emu) {
    delete_file(emu, "DeleteFileA", false);
}

fn delete_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = emu.regs().rcx;
    let path = strings::read_text(emu, path_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn DeleteFileW(emu: &mut emu::Emu) {
    delete_file(emu, "DeleteFileW", true);
}
