use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub(super) fn GetForegroundWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = helper::handler_create("hwnd://foreground");
    log_red!(emu, "user32!GetForegroundWindow => 0x{:x}", hwnd);
    call.set_return(emu, hwnd);
}
