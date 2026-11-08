use crate::emu;
use crate::winapi::helper;

pub fn FreeAddrInfoExW(emu: &mut emu::Emu) {
    let info_ptr = emu.regs().rcx;
    log_red!(emu, "ws2_32!FreeAddrInfoExW ptr:0x{:x}", info_ptr);
    helper::handler_close(info_ptr);
}
