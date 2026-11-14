use crate::emu;
use crate::winapi::common::strings;

pub fn GetLongPathNameA(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameA", false);
}

fn get_long_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let short = emu.regs().rcx;
    let long = emu.regs().rdx;
    let buf_len = emu.regs().r8 as u32;
    let path = strings::read_text(emu, short, wide);
    log_red!(emu, "kernel32!{api} '{}'", path);
    if long != 0 && buf_len > path.len() as u32 {
        strings::write_string(emu, long, &path, wide);
    }
    emu.regs_mut().rax = path.len() as u64;
}

pub fn GetLongPathNameW(emu: &mut emu::Emu) {
    get_long_path_name(emu, "GetLongPathNameW", true);
}
