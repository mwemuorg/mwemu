use crate::emu;
use crate::winapi::helper;

pub fn WSALookupServiceBeginA(emu: &mut emu::Emu) {
    let restrictions_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let handle_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!WSALookupServiceBeginA flags:0x{:x}", flags);
    let handle = helper::handler_create("wsa_lookup");
    if handle_ptr != 0 {
        emu.maps.write_qword(handle_ptr, handle);
    }
    emu.regs_mut().rax = 0;
}
