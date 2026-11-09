use crate::emu;
use crate::winapi::helper;

pub fn CoIsHandlerConnected(emu: &mut emu::Emu) {
    let punk = emu.regs().rcx;
    log_red!(emu, "ole32!CoIsHandlerConnected punk:0x{:x}", punk);
    let connected = if helper::handler_exist(punk) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = connected;
}
