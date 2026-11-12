use crate::emu;
use crate::winapi::common::strings;

pub fn SetFileAttributesA(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesA", false);
}

fn set_file_attributes(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path = strings::read_text(emu, emu.regs().rcx, wide);
    let attrs = emu.regs().rdx;
    log_red!(emu, "kernel32!{api} '{}' attrs:0x{:x}", path, attrs);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetFileAttributesW(emu: &mut emu::Emu) {
    set_file_attributes(emu, "SetFileAttributesW", true);
}
