use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAStringToAddressA(emu: &mut emu::Emu) {
    wsa_string_to_address(emu, "WSAStringToAddressA", false);
}

fn wsa_string_to_address(emu: &mut emu::Emu, api: &str, wide: bool) {
    let addr_str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let family = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let proto_info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let sockaddr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let addr_str = if addr_str_ptr != 0 {
        strings::read_text(emu, addr_str_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!{api} addr:'{}' family:{}", addr_str, family);
    if sockaddr_ptr != 0 && family == 2 {
        emu.maps.write_word(sockaddr_ptr, 2);
        emu.maps.write_word(sockaddr_ptr + 2, 0);
        emu.maps.write_dword(sockaddr_ptr + 4, 0x0100007f);
    }
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

pub fn WSAStringToAddressW(emu: &mut emu::Emu) {
    wsa_string_to_address(emu, "WSAStringToAddressW", true);
}
