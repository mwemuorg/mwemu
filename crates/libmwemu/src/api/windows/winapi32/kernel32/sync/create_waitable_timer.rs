use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateWaitableTimerA(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerA", false);
}

fn create_waitable_timer(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _lpTimerAttributes = api_try!(emu, call.arg_u32(emu, 0));
    let _bManualReset = api_try!(emu, call.arg_u32(emu, 1));
    let lpTimerName = api_try!(emu, call.arg_u32(emu, 2));

    let lpTimerName = strings::read_text(emu, lpTimerName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpTimerName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn CreateWaitableTimerW(emu: &mut emu::Emu) {
    create_waitable_timer(emu, "CreateWaitableTimerW", true);
}
