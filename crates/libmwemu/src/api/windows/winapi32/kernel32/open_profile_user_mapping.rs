use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn OpenProfileUserMapping(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!OpenProfileUserMapping");
    call.set_return(emu, helper::handler_create("OpenProfileUserMapping"));
}
