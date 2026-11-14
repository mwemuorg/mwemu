use crate::emu;
use crate::winapi::helper;

pub fn FreeAddrInfoEx(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "ws2_32!FreeAddrInfoEx handle:0x{:x}", handle);
    helper::handler_close(handle);
}

pub fn FreeAddrInfoExW(emu: &mut emu::Emu) {
    let info_ptr = emu.regs().rcx;
    log_red!(emu, "ws2_32!FreeAddrInfoExW ptr:0x{:x}", info_ptr);
    helper::handler_close(info_ptr);
}
