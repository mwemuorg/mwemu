use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateWaitableTimerExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _lpTimerAttributes = api_try!(emu, call.arg_u32(emu, 0));
    let lpTimerName = api_try!(emu, call.arg_u32(emu, 1));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 2));
    let _dwDesiredAccess = api_try!(emu, call.arg_u32(emu, 3));

    let lpTimerName = emu.maps.read_string(lpTimerName as u64);

    log_red!(emu, "kernel32!CreateWaitableTimerExA {}", lpTimerName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn CreateWaitableTimerExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 4);
    log_red!(emu, "kernel32!CreateWaitableTimerExW");
    call.set_return(emu, helper::handler_create("CreateWaitableTimerExW"));
}
