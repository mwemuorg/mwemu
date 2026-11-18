use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn CreateEventExA(emu: &mut emu::Emu) {
    create_event_ex(emu, "CreateEventExA");
}

fn create_event_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 4);
    log_red!(emu, "kernel32!{api}");
    call.set_return(emu, helper::handler_create(api));
}

pub fn CreateEventExW(emu: &mut emu::Emu) {
    create_event_ex(emu, "CreateEventExW");
}
