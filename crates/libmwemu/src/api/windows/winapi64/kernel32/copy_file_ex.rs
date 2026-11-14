use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn CopyFileExA(emu: &mut Emu) {
    invoke(emu, false, "CopyFileExA");
}

pub fn CopyFileExW(emu: &mut Emu) {
    invoke(emu, true, "CopyFileExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let source = strings::read_text(emu, emu.regs().rcx, wide);
    let destination = strings::read_text(emu, emu.regs().rdx, wide);
    log_red!(emu, "kernel32!{} '{}' -> '{}'", api, source, destination);
    emu.regs_mut().rax = 1;
}
