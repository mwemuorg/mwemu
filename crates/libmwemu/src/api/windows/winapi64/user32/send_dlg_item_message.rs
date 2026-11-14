use crate::emu;
use crate::winapi::common::strings;

pub fn SendDlgItemMessageA(emu: &mut emu::Emu) {
    send_dlg_item_message(emu, "SendDlgItemMessageA", false);
}

fn send_dlg_item_message(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let s = strings::read_text(emu, str_ptr, wide);
    log_red!(emu, "user32!{api} '{}'  0x{:x}", s, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SendDlgItemMessageW(emu: &mut emu::Emu) {
    send_dlg_item_message(emu, "SendDlgItemMessageW", true);
}
