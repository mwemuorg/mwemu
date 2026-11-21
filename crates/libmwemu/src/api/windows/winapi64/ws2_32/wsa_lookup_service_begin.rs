use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSALookupServiceBeginA(emu: &mut emu::Emu) {
    wsa_lookup_service_begin(emu, "WSALookupServiceBeginA");
}

fn wsa_lookup_service_begin(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let restrictions_ptr = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let handle_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!{api} flags:0x{:x}", flags);
    let handle = helper::handler_create("wsa_lookup");
    if handle_ptr != 0 {
        emu.maps.write_qword(handle_ptr, handle);
    }
    call.set_return(emu, 0);
}

pub fn WSALookupServiceBeginW(emu: &mut emu::Emu) {
    wsa_lookup_service_begin(emu, "WSALookupServiceBeginW");
}
