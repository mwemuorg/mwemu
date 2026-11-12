use crate::emu;
use crate::winapi::common::strings;

pub fn AddFontResourceExA(emu: &mut emu::Emu) {
    add_font_resource_ex(emu, "AddFontResourceExA", false);
}

fn add_font_resource_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name = strings::read_text(emu, emu.regs().rcx, wide);
    let fl = emu.regs().rdx;
    log_red!(emu, "gdi32!{api} '{}' flags:0x{:x}", name, fl);
    emu.regs_mut().rax = 1;
}

pub fn AddFontResourceExW(emu: &mut emu::Emu) {
    add_font_resource_ex(emu, "AddFontResourceExW", true);
}
