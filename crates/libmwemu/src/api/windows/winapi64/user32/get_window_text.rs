use crate::emu;

pub(super) fn GetWindowTextA(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextA", false);
}

pub(super) fn GetWindowTextW(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextW", true);
}

fn get_window_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let max_count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "user32!{api} hwnd=0x{:x} buf=0x{:x} max={}",
        hwnd,
        buf,
        max_count
    );
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
