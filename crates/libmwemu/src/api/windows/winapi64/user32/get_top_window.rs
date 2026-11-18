use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetTopWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!GetTopWindow arg1:0x{:x}", arg1);
    let hndl = helper::handler_create("user32://GetTopWindow");
    call.set_return(emu, hndl);
}
