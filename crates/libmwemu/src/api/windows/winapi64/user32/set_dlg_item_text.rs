use crate::emu;
use crate::winapi::common::strings;

pub fn SetDlgItemTextA(emu: &mut emu::Emu) {
    set_dlg_item_text(emu, "SetDlgItemTextA", false);
}

fn set_dlg_item_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hdlg = emu.regs().rcx;
    let id = emu.regs().rdx as u32;
    let text_ptr = emu.regs().r8;
    let text = if text_ptr != 0 {
        strings::read_text(emu, text_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "user32!{api} id:{} '{}'", id, text);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetDlgItemTextW(emu: &mut emu::Emu) {
    set_dlg_item_text(emu, "SetDlgItemTextW", true);
}
