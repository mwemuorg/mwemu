use crate::emu;
use crate::winapi::helper;

pub fn GetDlgItem(emu: &mut emu::Emu) {
    let hdlg = emu.regs().rcx;
    let id = emu.regs().rdx as u32;
    let hwnd = helper::handler_create("dlg_item");
    log_red!(
        emu,
        "user32!GetDlgItem dlg:0x{:x} id:{} => 0x{:x}",
        hdlg,
        id,
        hwnd
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hwnd;
}
