use crate::emu;

pub fn MessageBoxTimeoutA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let mb_type = emu.regs().r9;
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
    log_red!(emu, "user32!MessageBoxTimeoutA '{}': '{}'", caption, text);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn MessageBoxTimeoutW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let mb_type = emu.regs().r9;
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
    log_red!(emu, "user32!MessageBoxTimeoutW '{}': '{}'", caption, text);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
