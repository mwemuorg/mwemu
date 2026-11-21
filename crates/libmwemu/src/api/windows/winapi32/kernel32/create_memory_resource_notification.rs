use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateMemoryResourceNotification(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 1);
    log_red!(emu, "kernel32!CreateMemoryResourceNotification");
    call.set_return(
        emu,
        helper::handler_create("CreateMemoryResourceNotification"),
    );
}
