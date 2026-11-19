use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateCompatibleDC(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "gdi32!CreateCompatibleDC hdc:0x{:x}", hdc);
    call.set_return(emu, helper::handler_create("gdi32://CreateCompatibleDC"));
}
