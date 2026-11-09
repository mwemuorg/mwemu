use crate::emu;

pub fn MessageBoxExA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let mtype = emu.regs().r9 as u32;
    let _lang_id = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let text = if text_ptr != 0 {
        emu.maps.read_string(text_ptr)
    } else {
        String::new()
    };
    let caption = if caption_ptr != 0 {
        emu.maps.read_string(caption_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!MessageBoxExA '{}' '{}' type:0x{:x}",
        caption,
        text,
        mtype
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // IDOK
}

pub fn MessageBoxExW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let mtype = emu.regs().r9 as u32;
    let _lang_id = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let text = if text_ptr != 0 {
        emu.maps.read_wide_string(text_ptr)
    } else {
        String::new()
    };
    let caption = if caption_ptr != 0 {
        emu.maps.read_wide_string(caption_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!MessageBoxExW '{}' '{}' type:0x{:x}",
        caption,
        text,
        mtype
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // IDOK
}
