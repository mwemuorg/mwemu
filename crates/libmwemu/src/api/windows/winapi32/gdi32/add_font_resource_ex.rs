use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn AddFontResourceExA(emu: &mut emu::Emu) {
    add_font_resource_ex(emu, "AddFontResourceExA", false);
}

fn add_font_resource_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = arg(emu, 0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "gdi32!{api} '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn AddFontResourceExW(emu: &mut emu::Emu) {
    add_font_resource_ex(emu, "AddFontResourceExW", true);
}
