use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn WSASetServiceA(emu: &mut emu::Emu) {
    wsa_set_service(emu, "WSASetServiceA", false);
}

fn wsa_set_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let query_set_ptr = api_try!(emu, call.arg(emu, 0));
    let operation = api_try!(emu, call.arg_u32(emu, 1));
    let flags = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(emu, "ws2_32!{api} op:{} flags:0x{:x}", operation, flags);
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
    call.cleanup_args(emu, 3);
}

pub fn WSASetServiceW(emu: &mut emu::Emu) {
    wsa_set_service(emu, "WSASetServiceW", true);
}
