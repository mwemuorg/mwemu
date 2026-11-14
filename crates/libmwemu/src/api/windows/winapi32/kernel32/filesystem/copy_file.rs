use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn CopyFileA(emu: &mut emu::Emu) {
    copy_file(emu, "CopyFileA", false);
}

fn copy_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src_ptr = WinApiAbi::arg(emu, 0);
    let dst_ptr = WinApiAbi::arg(emu, 1);
    let do_fail = WinApiAbi::arg(emu, 2) as u32;

    let src = strings::read_text(emu, src_ptr, wide);
    let dst = strings::read_text(emu, dst_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}` to `{}`", src, dst);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

pub fn CopyFileW(emu: &mut emu::Emu) {
    copy_file(emu, "CopyFileW", true);
}
