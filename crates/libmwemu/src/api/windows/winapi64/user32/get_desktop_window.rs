use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub(super) fn GetDesktopWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h = helper::handler_create("desktop_window");
    log_red!(emu, "user32!GetDesktopWindow => 0x{:x}", h);
    call.set_return(emu, h);
}
