use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WakeConditionVariable(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let cv = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!{} cv:0x{:x}", api, cv);
}
