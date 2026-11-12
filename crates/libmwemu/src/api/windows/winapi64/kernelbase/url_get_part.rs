use crate::emu;
use crate::winapi::common::strings;

pub fn UrlGetPartA(emu: &mut emu::Emu) {
    url_get_part(emu, "UrlGetPartA", false);
}

fn url_get_part(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn UrlGetPartW(emu: &mut emu::Emu) {
    url_get_part(emu, "UrlGetPartW", true);
}
