use crate::emu;
use crate::winapi::common::strings;

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringA", false);
}

fn output_debug_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_ptr = emu.regs().rcx;

    let msg = strings::read_text(emu, str_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}`", msg);

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringW", true);
}
