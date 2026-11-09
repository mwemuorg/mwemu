use crate::emu;
use crate::winapi::helper;

pub fn CloseThreadpoolCleanupGroup(emu: &mut emu::Emu) {
    let group = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CloseThreadpoolCleanupGroup group:0x{:x}",
        group
    );
    helper::handler_close(group);
}
