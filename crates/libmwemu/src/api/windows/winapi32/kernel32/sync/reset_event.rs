use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ResetEvent(emu: &mut emu::Emu) {
    let handle = arg(emu, 0);
    log_red!(emu, "kernel32!ResetEvent hndl: 0x{:x}", handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
