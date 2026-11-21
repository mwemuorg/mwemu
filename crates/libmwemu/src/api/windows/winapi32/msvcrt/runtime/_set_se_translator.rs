use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _set_se_translator(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let handler = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!{} handler:0x{:x}", api, handler);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
