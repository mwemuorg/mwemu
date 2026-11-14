use crate::emu;
use crate::winapi::common::strings;

pub fn MoveFileA(emu: &mut emu::Emu) {
    move_file(emu, "MoveFileA", false);
}

fn move_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;

    let src = strings::read_text(emu, src_ptr, wide);
    let dst = strings::read_text(emu, dst_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}` to `{}`", src, dst);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn MoveFileW(emu: &mut emu::Emu) {
    move_file(emu, "MoveFileW", true);
}
