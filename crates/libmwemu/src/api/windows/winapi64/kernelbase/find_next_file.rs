use crate::emu;
use crate::winapi::common::strings;

pub fn FindNextFileA(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileA", false);
}

fn find_next_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let name = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "kernelbase!{api} '{}' 0x{:x}", name, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FindNextFileW(emu: &mut emu::Emu) {
    find_next_file(emu, "FindNextFileW", true);
}
