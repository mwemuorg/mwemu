use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateEnclave(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!CreateEnclave");
    call.set_return(emu, helper::handler_create("CreateEnclave"));
    call.cleanup_args(emu, 8);
}
