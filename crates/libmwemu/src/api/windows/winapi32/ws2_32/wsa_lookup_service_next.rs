use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSALookupServiceNextA(emu: &mut emu::Emu) {
    wsa_lookup_service_next(emu, "WSALookupServiceNextA");
}

fn wsa_lookup_service_next(emu: &mut emu::Emu, api: &str) {
    let lookup_handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "ws2_32!{api} handle:0x{:x}", lookup_handle);
    emu.set_last_error(10110);
    emu.regs_mut().rax = 0xFFFFFFFF;
    stdcall_ret(emu, 4);
}

pub fn WSALookupServiceNextW(emu: &mut emu::Emu) {
    wsa_lookup_service_next(emu, "WSALookupServiceNextW");
}
