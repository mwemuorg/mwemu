use crate::emu;
use crate::winapi::common::strings;

pub fn RemoveFontResourceExA(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExA", false);
}

fn remove_font_resource_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name = strings::read_text(emu, emu.regs().rcx, wide);
    let fl = emu.regs().rdx;
    log_red!(emu, "gdi32!{api} '{}' flags:0x{:x}", name, fl);
    emu.regs_mut().rax = 1;
}

pub fn RemoveFontResourceExW(emu: &mut emu::Emu) {
    remove_font_resource_ex(emu, "RemoveFontResourceExW", true);
}
