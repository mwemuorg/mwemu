use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAEnumProtocolsA(emu: &mut emu::Emu) {
    wsa_enum_protocols(emu, "WSAEnumProtocolsA");
}

fn wsa_enum_protocols(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let protocols_ptr = api_try!(emu, call.arg(emu, 0));
    let buf_ptr = api_try!(emu, call.arg(emu, 1));
    let buf_len_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!{api}");
    if buf_len_ptr != 0 {
        emu.maps.write_dword(buf_len_ptr, 0);
    }
    call.set_return(emu, 0);
}

pub fn WSAEnumProtocolsW(emu: &mut emu::Emu) {
    wsa_enum_protocols(emu, "WSAEnumProtocolsW");
}
