use crate::emu;
use crate::winapi::common::strings;

pub fn RemoveDirectoryA(emu: &mut emu::Emu) {
    remove_directory(emu, "RemoveDirectoryA", false);
}

fn remove_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path = strings::read_text(emu, emu.regs().rcx, wide);
    log_red!(emu, "kernel32!{api} '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn RemoveDirectoryW(emu: &mut emu::Emu) {
    remove_directory(emu, "RemoveDirectoryW", true);
}
