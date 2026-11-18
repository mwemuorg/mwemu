use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateIoRing(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!CreateIoRing");
    call.set_return(emu, helper::handler_create("CreateIoRing"));
    call.cleanup_args(emu, 6);
}
