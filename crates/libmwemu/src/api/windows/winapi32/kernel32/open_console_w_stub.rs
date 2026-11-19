use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn OpenConsoleWStub(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!OpenConsoleWStub");
    call.set_return(emu, helper::handler_create("OpenConsoleWStub"));
}
