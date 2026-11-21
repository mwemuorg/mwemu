use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateCaret(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("CreateCaret");
    log_red!(emu, "user32!CreateCaret => 0x{:x}", hndl);
    call.set_return(emu, hndl);
}
