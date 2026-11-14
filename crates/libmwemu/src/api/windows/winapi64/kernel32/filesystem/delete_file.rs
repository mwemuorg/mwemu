use crate::emu;
use crate::winapi::common::strings;

pub fn DeleteFileA(emu: &mut emu::Emu) {
    delete_file(emu, "DeleteFileA", false);
}

fn delete_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let filename_ptr = emu.regs().rcx;

    let filename = strings::read_text(emu, filename_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}` (stub: not deleting)", filename);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // Success
}

pub fn DeleteFileW(emu: &mut emu::Emu) {
    delete_file(emu, "DeleteFileW", true);
}
