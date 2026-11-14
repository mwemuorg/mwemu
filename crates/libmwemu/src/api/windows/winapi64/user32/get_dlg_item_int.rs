use crate::emu;
use crate::winapi::helper;

pub fn GetDlgItemInt(emu: &mut emu::Emu) {
    let hdlg = emu.regs().rcx;
    let item_id = emu.regs().rdx;
    let hndl = helper::handler_create(&format!("dlgitem_{}", item_id));
    log_red!(
        emu,
        "user32!GetDlgItemInt dlg:0x{:x} id:{} => 0x{:x}",
        hdlg,
        item_id,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
