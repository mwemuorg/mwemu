use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn VerifyVersionInfoA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _lpVersionInformation = api_try!(emu, call.arg_u32(emu, 0));
    let _dwTypeMask = api_try!(emu, call.arg_u32(emu, 1));
    let _dwlConditionMask = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!VerifyVersionInfoA");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn VerifyVersionInfoW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!VerifyVersionInfoW");

    call.cleanup_args(emu, 4);
    call.set_return(emu, 0xffff);
}
