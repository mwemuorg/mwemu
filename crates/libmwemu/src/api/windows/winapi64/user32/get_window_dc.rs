use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetWindowDC(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let hdc = helper::handler_create("dc");
    log_red!(emu, "user32!GetWindowDC hwnd:0x{:x} => 0x{:x}", hwnd, hdc);
    emu.set_last_error(0);
    call.set_return(emu, hdc);
}
