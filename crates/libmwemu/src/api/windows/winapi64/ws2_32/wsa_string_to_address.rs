use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn WSAStringToAddressA(emu: &mut emu::Emu) {
    wsa_string_to_address(emu, "WSAStringToAddressA", false);
}

fn wsa_string_to_address(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let addr_str_ptr = api_try!(emu, call.arg(emu, 0));
    let family = api_try!(emu, call.arg_u32(emu, 1));
    let proto_info_ptr = api_try!(emu, call.arg(emu, 2));
    let sockaddr_ptr = api_try!(emu, call.arg(emu, 3));
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
    call.set_return(emu, 0);
}

pub fn WSAStringToAddressW(emu: &mut emu::Emu) {
    wsa_string_to_address(emu, "WSAStringToAddressW", true);
}
