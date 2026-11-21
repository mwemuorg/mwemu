use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn WaitNamedPipeA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpNamedPipeName = api_try!(emu, call.arg_u32(emu, 0));
    let _nTimeOut = api_try!(emu, call.arg_u32(emu, 1));

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!WaitNamedPipeA {}", lpNamedPipeName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn WaitNamedPipeW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "kernel32!WaitNamedPipeW 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
