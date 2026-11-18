use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub(super) fn GetFocus(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = helper::handler_create("hwnd://focus");
    log_red!(emu, "user32!GetFocus => 0x{:x}", hwnd);
    call.set_return(emu, hwnd);
}
