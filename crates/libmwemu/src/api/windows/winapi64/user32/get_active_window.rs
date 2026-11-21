use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetActiveWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!GetActiveWindow");
    emu.set_last_error(0);
    call.set_return(emu, helper::handler_create("hwnd://active"));
}
