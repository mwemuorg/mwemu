use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateTimerQueueTimer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 7);
    log_red!(emu, "kernel32!CreateTimerQueueTimer");
    call.set_return(emu, helper::handler_create("CreateTimerQueueTimer"));
}
