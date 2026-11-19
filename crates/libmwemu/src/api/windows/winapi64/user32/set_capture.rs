use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetCapture(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!SetCapture hwnd:0x{:x}", hwnd);
    if hwnd != 0 {
        let _ = helper::handler_create("Capture");
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
