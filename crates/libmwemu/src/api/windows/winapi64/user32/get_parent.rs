use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetParent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!GetParent hwnd:0x{:x}", hwnd);
    let parent = if helper::handler_exist(hwnd) {
        helper::handler_create("parent_hwnd")
    } else {
        0
    };
    emu.set_last_error(0);
    call.set_return(emu, parent);
}
