use crate::emu;

pub fn GetDlgItemTextA(emu: &mut emu::Emu) {
    get_dlg_item_text(emu, "GetDlgItemTextA", false);
}

pub fn GetDlgItemTextW(emu: &mut emu::Emu) {
    get_dlg_item_text(emu, "GetDlgItemTextW", true);
}

fn get_dlg_item_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _hdlg = emu.regs().rcx;
    let _id = emu.regs().rdx as u32;
    let buf = emu.regs().r8;
    let max_count = emu.regs().r9 as u32;
    log_red!(emu, "user32!{api} id:{} buf:0x{:x}", _id, buf);
    if wide {
        if buf != 0 && max_count > 0 {
            emu.maps.write_word(buf, 0);
        }
    } else {
        if buf != 0 && max_count > 0 {
            emu.maps.write_byte(buf, 0);
        }
    }

    emu.regs_mut().rax = 0;
}
