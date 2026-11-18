use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ClientToScreen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let pt = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!ClientToScreen hwnd:0x{:x} pt:0x{:x}", hwnd, pt);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
