use crate::emu;
use crate::winapi::common::strings;

pub fn MessageBoxIndirectA(emu: &mut emu::Emu) {
    message_box_indirect(emu, "MessageBoxIndirectA", false);
}

fn message_box_indirect(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let mb_type = emu.regs().r9;
    let text = if text_ptr != 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    let caption = if caption_ptr != 0 {
        strings::read_text(emu, caption_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "user32!{api} '{}': '{}'", caption, text);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn MessageBoxIndirectW(emu: &mut emu::Emu) {
    message_box_indirect(emu, "MessageBoxIndirectW", true);
}
