use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn TextOutA(emu: &mut emu::Emu) {
    text_out(emu, "TextOutA", false);
}

fn text_out(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let text_ptr = arg(emu, 3) as u64;
    let text = strings::read_text(emu, text_ptr, wide);
    log_red!(emu, "gdi32!{api} '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}

pub fn TextOutW(emu: &mut emu::Emu) {
    text_out(emu, "TextOutW", true);
}
