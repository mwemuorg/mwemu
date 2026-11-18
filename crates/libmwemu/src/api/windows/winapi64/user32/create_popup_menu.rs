use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreatePopupMenu(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!CreatePopupMenu");
    call.set_return(emu, helper::handler_create("CreatePopupMenu"));
}
