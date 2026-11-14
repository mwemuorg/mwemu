use crate::emu;
use crate::winapi::common::strings;

pub fn GetShortPathNameA(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameA", false);
}

fn get_short_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let long = emu.regs().rcx;
    let short = emu.regs().rdx;
    let buf_len = emu.regs().r8 as u32;
    let path = strings::read_text(emu, long, wide);
    log_red!(emu, "kernel32!{api} '{}'", path);
    if short != 0 && buf_len > path.len() as u32 {
        strings::write_string(emu, short, &path, wide);
    }
    emu.regs_mut().rax = path.len() as u64;
}

pub fn GetShortPathNameW(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameW", true);
}
