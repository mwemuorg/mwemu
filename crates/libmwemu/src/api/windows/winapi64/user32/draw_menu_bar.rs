use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DrawMenuBar(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!DrawMenuBar hwnd:0x{:x}", hwnd);
    let result = if helper::handler_exist(hwnd) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    call.set_return(emu, result);
}
