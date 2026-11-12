use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn FindWindowA(emu: &mut emu::Emu) {
    find_window(emu, "FindWindowA", false);
}

fn find_window(emu: &mut emu::Emu, api: &str, wide: bool) {
    let class_ptr = arg(emu, 0) as u64;
    let window_ptr = arg(emu, 1) as u64;
    let class_name = if class_ptr != 0 {
        strings::read_text(emu, class_ptr, wide)
    } else {
        String::new()
    };
    let window_name = if window_ptr != 0 {
        strings::read_text(emu, window_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!{api} class='{}' window='{}'",
        class_name,
        window_name
    );
    emu.regs_mut().rax = 0; // not found
    stdcall_ret(emu, 2);
}

pub(super) fn FindWindowW(emu: &mut emu::Emu) {
    find_window(emu, "FindWindowW", true);
}
