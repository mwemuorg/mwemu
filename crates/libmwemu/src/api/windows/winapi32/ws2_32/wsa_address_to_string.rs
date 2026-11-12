use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAAddressToStringA(emu: &mut emu::Emu) {
    wsa_address_to_string(emu, "WSAAddressToStringA", false);
}

fn wsa_address_to_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let addr_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let addr_len = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!{api} addr:0x{:x}", addr_ptr);
    if addr_ptr != 0 && str_ptr != 0 {
        let ip = emu.maps.read_dword(addr_ptr + 4).unwrap_or(0x0100007f);
        let port = emu.maps.read_word(addr_ptr + 2).unwrap_or(0).to_be();
        let s = format!(
            "{}.{}.{}.{}:{}",
            ip & 0xFF,
            (ip >> 8) & 0xFF,
            (ip >> 16) & 0xFF,
            (ip >> 24) & 0xFF,
            port
        );
        strings::write_string(emu, str_ptr, &s, wide);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

pub fn WSAAddressToStringW(emu: &mut emu::Emu) {
    wsa_address_to_string(emu, "WSAAddressToStringW", true);
}
