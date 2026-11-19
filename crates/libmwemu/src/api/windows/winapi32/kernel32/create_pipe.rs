use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreatePipe(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 4);
    log_red!(emu, "kernel32!CreatePipe");
    call.set_return(emu, helper::handler_create("CreatePipe"));
}
