use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _crt_atexit(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Cdecl);
    let func_ptr = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!{} func:0x{:x}", api, func_ptr);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
