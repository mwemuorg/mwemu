use crate::emu;
use crate::winapi::helper;

pub fn FreeAddrInfoEx(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "ws2_32!FreeAddrInfoEx handle:0x{:x}", handle);
    helper::handler_close(handle);
}
