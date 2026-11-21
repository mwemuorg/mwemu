use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn clock(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!clock 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    call.set_return(emu, 1);
}
