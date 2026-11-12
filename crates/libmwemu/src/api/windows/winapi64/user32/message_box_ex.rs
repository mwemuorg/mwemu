use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn MessageBoxExA(emu: &mut emu::Emu) {
    message_box_ex(emu, "MessageBoxExA", false);
}

fn message_box_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let mtype = emu.regs().r9 as u32;
    let _lang_id = WinApiAbi::arg(emu, 4);
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
        "user32!{api} '{}' '{}' type:0x{:x}",
        caption,
        text,
        mtype
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // IDOK
}

pub fn MessageBoxExW(emu: &mut emu::Emu) {
    message_box_ex(emu, "MessageBoxExW", true);
}
