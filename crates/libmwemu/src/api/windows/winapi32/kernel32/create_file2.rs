use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateFile2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 5);
    log_red!(emu, "kernel32!CreateFile2");
    call.set_return(emu, helper::handler_create("CreateFile2"));
}
