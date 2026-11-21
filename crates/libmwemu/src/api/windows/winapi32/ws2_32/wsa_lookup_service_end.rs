use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSALookupServiceEnd(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lookup_handle = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "ws2_32!WSALookupServiceEnd handle:0x{:x}",
        lookup_handle
    );
    if helper::handler_exist(lookup_handle) {
        helper::handler_close(lookup_handle);
        call.set_return(emu, 0);
    } else {
        emu.set_last_error(6);
        call.set_return(emu, 0xFFFFFFFF);
    }
    call.cleanup_args(emu, 1);
}
