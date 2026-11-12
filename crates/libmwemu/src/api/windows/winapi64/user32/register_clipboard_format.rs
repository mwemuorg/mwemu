use crate::emu;
use crate::winapi::common::strings;

pub fn RegisterClipboardFormatA(emu: &mut emu::Emu) {
    register_clipboard_format(emu, "RegisterClipboardFormatA", false);
}

fn register_clipboard_format(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xc000;
}

pub fn RegisterClipboardFormatW(emu: &mut emu::Emu) {
    register_clipboard_format(emu, "RegisterClipboardFormatW", true);
}
