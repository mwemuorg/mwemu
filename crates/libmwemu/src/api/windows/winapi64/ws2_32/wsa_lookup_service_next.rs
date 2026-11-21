use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSALookupServiceNextA(emu: &mut emu::Emu) {
    wsa_lookup_service_next(emu, "WSALookupServiceNextA");
}

fn wsa_lookup_service_next(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let lookup_handle = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "ws2_32!{api} handle:0x{:x}", lookup_handle);
    emu.set_last_error(10110);
    call.set_return(emu, 0xFFFFFFFF);
}

pub fn WSALookupServiceNextW(emu: &mut emu::Emu) {
    wsa_lookup_service_next(emu, "WSALookupServiceNextW");
}
