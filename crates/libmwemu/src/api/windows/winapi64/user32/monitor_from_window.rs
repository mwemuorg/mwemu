use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn MonitorFromWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let hmon = helper::handler_create("monitor");
    log_red!(
        emu,
        "user32!MonitorFromWindow hwnd:0x{:x} flags:0x{:x} => 0x{:x}",
        hwnd,
        flags,
        hmon
    );
    emu.set_last_error(0);
    call.set_return(emu, hmon);
}
