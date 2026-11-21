use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn __setusermatherr(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let handler = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!__setusermatherr handler:0x{:x}", handler);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
