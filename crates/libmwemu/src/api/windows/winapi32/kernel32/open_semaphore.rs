use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn OpenSemaphoreA(emu: &mut emu::Emu) {
    open_semaphore(emu, "OpenSemaphoreA");
}

fn open_semaphore(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 3);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn OpenSemaphoreW(emu: &mut emu::Emu) {
    open_semaphore(emu, "OpenSemaphoreW");
}
