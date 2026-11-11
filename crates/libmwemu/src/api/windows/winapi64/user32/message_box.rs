use crate::emu;

pub(super) fn MessageBoxA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let utype = emu.regs().r9;

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
        "user32!MessageBoxA hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
        hwnd,
        caption,
        text,
        utype
    );

    emu.regs_mut().rax = 1; // IDOK
}

pub fn MessageBoxW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let utype = emu.regs().r9;

    let text = emu.maps.read_wide_string(text_ptr);
    let caption = if caption_ptr != 0 {
        emu.maps.read_wide_string(caption_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "user32!MessageBoxW hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
        hwnd,
        caption,
        text,
        utype
    );

    emu.regs_mut().rax = 1; // IDOK
}
