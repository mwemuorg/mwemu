use crate::emu;

pub fn SetDlgItemTextA(emu: &mut emu::Emu) {
    let hdlg = emu.regs().rcx;
    let id = emu.regs().rdx as u32;
    let text_ptr = emu.regs().r8;
    let text = if text_ptr != 0 {
        emu.maps.read_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "user32!SetDlgItemTextA id:{} '{}'", id, text);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetDlgItemTextW(emu: &mut emu::Emu) {
    let hdlg = emu.regs().rcx;
    let id = emu.regs().rdx as u32;
    let text_ptr = emu.regs().r8;
    let text = if text_ptr != 0 {
        emu.maps.read_wide_string(text_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "user32!SetDlgItemTextW id:{} '{}'", id, text);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
