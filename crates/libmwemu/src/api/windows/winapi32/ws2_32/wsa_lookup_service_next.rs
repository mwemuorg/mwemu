use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSALookupServiceNextA(emu: &mut emu::Emu) {
    wsa_lookup_service_next(emu, "WSALookupServiceNextA");
}

fn wsa_lookup_service_next(emu: &mut emu::Emu, api: &str) {
    let lookup_handle = WinApiAbi::arg(emu, 0);
    let flags = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "ws2_32!{api} handle:0x{:x}", lookup_handle);
    emu.set_last_error(10110);
    emu.regs_mut().rax = 0xFFFFFFFF;
    stdcall_ret(emu, 4);
}

pub fn WSALookupServiceNextW(emu: &mut emu::Emu) {
    wsa_lookup_service_next(emu, "WSALookupServiceNextW");
}
