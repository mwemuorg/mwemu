use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CreateWindowExA(emu: &mut emu::Emu) {
    create_window_ex(emu, "CreateWindowExA", false);
}

fn create_window_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let class = strings::read_text(emu, arg(emu, 1) as u64, wide);
    let title = strings::read_text(emu, arg(emu, 2) as u64, wide);
    log_red!(emu, "user32!{api} class:'{}' title:'{}'", class, title);
    emu.regs_mut().rax = 0x1337_0001;
    stdcall_ret(emu, 12);
}

pub(super) fn CreateWindowExW(emu: &mut emu::Emu) {
    create_window_ex(emu, "CreateWindowExW", true);
}
