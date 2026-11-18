use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _beginthread(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let start = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "msvcrt!_beginthread start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthread");
    call.set_return(emu, hndl);
}
