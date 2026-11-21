use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _EH_prolog(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!{} 0x{:x}", api, arg1);
    helper::advance_tick(emu, 1);
    call.set_return(emu, 0);
}
