use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn FormatMessageA(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageA");
}

fn format_message(emu: &mut emu::Emu, api: &str) {
    let flags = arg(emu, 0);
    log_red!(emu, "kernel32!{api} flags: 0x{:x} (stub)", flags);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 7);
}

pub fn FormatMessageW(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageW");
}
