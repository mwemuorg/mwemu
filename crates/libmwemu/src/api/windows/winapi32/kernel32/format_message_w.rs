use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FormatMessageW(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    log_red!(emu, "kernel32!FormatMessageW flags: 0x{:x} (stub)", flags);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 7);
}
