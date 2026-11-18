use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateWaitableTimerExA(emu: &mut Emu) {
    invoke(emu, false, "CreateWaitableTimerExA");
}

pub fn CreateWaitableTimerExW(emu: &mut Emu) {
    invoke(emu, true, "CreateWaitableTimerExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let name = strings::read_text(emu, api_try!(emu, call.arg(emu, 1)), wide);
    log_red!(emu, "kernel32!{} name:'{}'", api, name);
    call.set_return(emu, helper::handler_create("waitable_timer"));
}
