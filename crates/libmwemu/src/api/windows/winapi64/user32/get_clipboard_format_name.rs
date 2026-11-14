use crate::emu;
use crate::winapi::common::strings;

pub fn GetClipboardFormatNameA(emu: &mut emu::Emu) {
    get_clipboard_format_name(emu, "GetClipboardFormatNameA", false);
}

fn get_clipboard_format_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
}

pub fn GetClipboardFormatNameW(emu: &mut emu::Emu) {
    get_clipboard_format_name(emu, "GetClipboardFormatNameW", true);
}
