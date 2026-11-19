use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSACloseEvent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let event = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WSACloseEvent event: 0x{:x}", event);
    if event != 0 {
        helper::handler_close(event);
    }
    call.set_return(emu, 1); // TRUE
}
