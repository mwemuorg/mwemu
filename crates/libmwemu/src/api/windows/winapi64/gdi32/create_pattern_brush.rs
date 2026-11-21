use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreatePatternBrush(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hbmp = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "gdi32!CreatePatternBrush hbmp:0x{:x}", hbmp);
    call.set_return(emu, helper::handler_create("gdi32://brush"));
}
