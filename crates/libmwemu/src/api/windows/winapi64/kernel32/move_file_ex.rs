use crate::emu;
use crate::winapi::common::strings;

pub fn MoveFileExA(emu: &mut emu::Emu) {
    move_file_ex(emu, "MoveFileExA", false);
}

fn move_file_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    let src_s = strings::read_text(emu, src, wide);
    let dst_s = strings::read_text(emu, dst, wide);
    log_red!(emu, "kernel32!{api} '{}' -> '{}'", src_s, dst_s);
    emu.regs_mut().rax = 1;
}

pub fn MoveFileExW(emu: &mut emu::Emu) {
    move_file_ex(emu, "MoveFileExW", true);
}
