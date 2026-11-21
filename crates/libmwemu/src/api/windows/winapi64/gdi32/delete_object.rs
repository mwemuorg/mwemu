use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DeleteObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let obj = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "gdi32!DeleteObject obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::handler_close(obj);
    }
    call.set_return(emu, 1);
}
