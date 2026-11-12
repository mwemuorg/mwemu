use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSALookupServiceBeginA(emu: &mut emu::Emu) {
    wsa_lookup_service_begin(emu, "WSALookupServiceBeginA");
}

fn wsa_lookup_service_begin(emu: &mut emu::Emu, api: &str) {
    let restrictions_ptr = WinApiAbi::arg(emu, 0);
    let flags = WinApiAbi::arg(emu, 1) as u32;
    let handle_ptr = WinApiAbi::arg(emu, 2);
    log_red!(emu, "ws2_32!{api} flags:0x{:x}", flags);
    let handle = helper::handler_create("wsa_lookup");
    if handle_ptr != 0 {
        emu.maps.write_dword(handle_ptr, handle as u32);
    }
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSALookupServiceBeginW(emu: &mut emu::Emu) {
    wsa_lookup_service_begin(emu, "WSALookupServiceBeginW");
}
