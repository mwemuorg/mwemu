use crate::emu;
use crate::winapi::common::strings;

pub fn PathIsValidCharA(emu: &mut emu::Emu) {
    path_is_valid_char(emu, "PathIsValidCharA", false);
}

fn path_is_valid_char(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!{api} '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn PathIsValidCharW(emu: &mut emu::Emu) {
    path_is_valid_char(emu, "PathIsValidCharW", true);
}
