use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAResetEvent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let event = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WSAResetEvent event:0x{:x}", event);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
