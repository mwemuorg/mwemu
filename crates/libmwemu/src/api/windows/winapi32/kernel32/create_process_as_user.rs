use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserA");
}

fn create_process_as_user(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 11);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserW");
}
