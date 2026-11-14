use crate::emu;
use crate::winapi::common::strings;

pub(super) fn MessageBoxA(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxA", false);
}

pub fn MessageBoxW(emu: &mut emu::Emu) {
    message_box(emu, "MessageBoxW", true);
}

fn message_box(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let utype = emu.regs().r9;
    if wide {
        let text = strings::read_text(emu, text_ptr, wide);
        let caption = if caption_ptr != 0 {
            strings::read_text(emu, caption_ptr, wide)
        } else {
            String::new()
        };
        log_red!(
            emu,
            "user32!{api} hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
            hwnd,
            caption,
            text,
            utype
        );
        emu.regs_mut().rax = 1;
    } else {
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
        log_red!(
            emu,
            "user32!{api} hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
            hwnd,
            caption,
            text,
            utype
        );
        emu.regs_mut().rax = 1;
    }
}
