use crate::emu;
use crate::winapi::helper;

pub fn WSALookupServiceBeginA(emu: &mut emu::Emu) {
    wsa_lookup_service_begin(emu, "WSALookupServiceBeginA");
}

fn wsa_lookup_service_begin(emu: &mut emu::Emu, api: &str) {
    let restrictions_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let handle_ptr = emu.regs().r8;
    log_red!(emu, "ws2_32!{api} flags:0x{:x}", flags);
    let handle = helper::handler_create("wsa_lookup");
    if handle_ptr != 0 {
        emu.maps.write_qword(handle_ptr, handle);
    }
    emu.regs_mut().rax = 0;
}

pub fn WSALookupServiceBeginW(emu: &mut emu::Emu) {
    wsa_lookup_service_begin(emu, "WSALookupServiceBeginW");
}
