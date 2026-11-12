use crate::emu;
use crate::winapi::common::strings;

pub fn GetWindowTextLengthA(emu: &mut emu::Emu) {
    get_window_text_length(emu, "GetWindowTextLengthA", false);
}

fn get_window_text_length(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hwnd = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let max_count = emu.regs().r8;
    let title = "mwemu_window";
    if buf_ptr != 0 && max_count > 0 {
        strings::write_string(emu, buf_ptr, title, wide);
    }
    log_red!(emu, "user32!{api} hwnd:0x{:x} => '{}'", hwnd, title);

    emu.set_last_error(0);
    emu.regs_mut().rax = title.len() as u64;
}

pub fn GetWindowTextLengthW(emu: &mut emu::Emu) {
    get_window_text_length(emu, "GetWindowTextLengthW", true);
}
