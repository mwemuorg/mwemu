use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn __setusermatherr(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let handler = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!{} handler:0x{:x}", api, handler);
    call.set_return(emu, 0);
    helper::advance_tick(emu, 1);
}
