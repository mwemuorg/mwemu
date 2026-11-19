use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessInternalA(emu: &mut emu::Emu) {
    create_process_internal(emu, "CreateProcessInternalA");
}

fn create_process_internal(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn CreateProcessInternalW(emu: &mut emu::Emu) {
    create_process_internal(emu, "CreateProcessInternalW");
}
