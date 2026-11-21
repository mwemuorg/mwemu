use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn OpenMutexA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _dwDesiredAccess = api_try!(emu, call.arg_u32(emu, 0));
    let _bInheritHandle = api_try!(emu, call.arg_u32(emu, 1));
    let lpName = api_try!(emu, call.arg_u32(emu, 2));

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!OpenMutexA {}", lpName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn OpenMutexW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 3);
    log_red!(emu, "kernel32!OpenMutexW");
    call.set_return(emu, helper::handler_create("OpenMutexW"));
}
