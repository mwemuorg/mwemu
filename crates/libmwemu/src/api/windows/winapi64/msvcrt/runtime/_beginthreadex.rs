use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _beginthreadex(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let _security = api_try!(emu, call.arg(emu, 0));
    let _stack_size = api_try!(emu, call.arg(emu, 1));
    let start = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "msvcrt!_beginthreadex start:0x{:x}", start);
    let hndl = helper::handler_create("msvcrt://_beginthreadex");
    call.set_return(emu, hndl);
}
