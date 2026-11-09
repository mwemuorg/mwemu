use crate::emu;
use crate::winapi::helper;

pub fn ClosePrivateNamespace(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!ClosePrivateNamespace h:0x{:x} flags:0x{:x}",
        handle,
        flags
    );
    helper::handler_close(handle);
    emu.regs_mut().rax = 1; // TRUE
}
