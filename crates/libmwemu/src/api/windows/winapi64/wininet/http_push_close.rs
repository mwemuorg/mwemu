use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn HttpPushClose(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    helper::handler_close(handle);
    log_red!(emu, "wininet!HttpPushClose 0x{:x}", handle);
    call.set_return(emu, 1);
}
