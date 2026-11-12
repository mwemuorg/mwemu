use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn OutputDebugStringA(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringA", false);
}

fn output_debug_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let msg = strings::read_text(emu, arg(emu, 0) as u64, wide);
    log_red!(emu, "kernel32!{api} `{}`", msg);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

pub fn OutputDebugStringW(emu: &mut emu::Emu) {
    output_debug_string(emu, "OutputDebugStringW", true);
}
