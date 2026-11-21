use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ResetEvent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let event = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!ResetEvent event:0x{:x}", event);
    if helper::handler_exist(event) {
        call.set_return(emu, 1); // TRUE
    } else {
        call.set_return(emu, 0); // FALSE
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
    }
}
