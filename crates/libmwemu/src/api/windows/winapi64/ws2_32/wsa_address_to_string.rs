use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn WSAAddressToStringA(emu: &mut emu::Emu) {
    wsa_address_to_string(emu, "WSAAddressToStringA", false);
}

fn wsa_address_to_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let addr_ptr = api_try!(emu, call.arg(emu, 0));
    let addr_len = api_try!(emu, call.arg_u32(emu, 1));
    let proto_info_ptr = api_try!(emu, call.arg(emu, 2));
    let str_ptr = api_try!(emu, call.arg(emu, 3));
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
    call.set_return(emu, 0);
}

pub fn WSAAddressToStringW(emu: &mut emu::Emu) {
    wsa_address_to_string(emu, "WSAAddressToStringW", true);
}
