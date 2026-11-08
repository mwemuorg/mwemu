use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ClosePrivateNamespace(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let flags = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!ClosePrivateNamespace h:0x{:x} flags:0x{:x}",
        handle,
        flags
    );
    helper::handler_close(handle);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 2);
}
