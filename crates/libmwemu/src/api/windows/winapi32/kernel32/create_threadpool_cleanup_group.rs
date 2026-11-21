use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolCleanupGroup(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!CreateThreadpoolCleanupGroup");
    call.set_return(emu, helper::handler_create("CreateThreadpoolCleanupGroup"));
}
