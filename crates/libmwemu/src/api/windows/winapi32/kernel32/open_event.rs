use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn OpenEventA(emu: &mut emu::Emu) {
    open_event(emu, "OpenEventA");
}

fn open_event(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 3);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn OpenEventW(emu: &mut emu::Emu) {
    open_event(emu, "OpenEventW");
}
