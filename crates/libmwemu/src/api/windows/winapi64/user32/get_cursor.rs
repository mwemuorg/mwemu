use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetCursor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "user32!GetCursor");
    let hndl = helper::handler_create("user32://cursor");
    call.set_return(emu, hndl);
}
