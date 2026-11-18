use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateSemaphoreExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _lpSemaphoreAttributes = api_try!(emu, call.arg_u32(emu, 0));
    let _lInitialCount = api_try!(emu, call.arg_u32(emu, 1));
    let _lMaximumCount = api_try!(emu, call.arg_u32(emu, 2));
    let lpName = api_try!(emu, call.arg_u32(emu, 3));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 4));
    let _dwDesiredAccess = api_try!(emu, call.arg_u32(emu, 5));

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateSemaphoreExA {}", lpName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn CreateSemaphoreExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 6);
    log_red!(emu, "kernel32!CreateSemaphoreExW");
    call.set_return(emu, helper::handler_create("CreateSemaphoreExW"));
}
