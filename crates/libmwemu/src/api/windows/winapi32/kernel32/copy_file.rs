use crate::emu;
use crate::winapi::common::strings;

pub fn CopyFileA(emu: &mut emu::Emu) {
    copy_file(emu, "CopyFileA", false);
}

fn copy_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let do_fail = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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
