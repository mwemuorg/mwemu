use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn QuirkIsEnabledForPackage4Worker(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _userSid = api_try!(emu, call.arg_u32(emu, 0));
    let _packageFullName = api_try!(emu, call.arg_u32(emu, 1));
    let _int = api_try!(emu, call.arg_u32(emu, 2));
    let _int = api_try!(emu, call.arg_u32(emu, 3));
    let _int = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "kernel32!QuirkIsEnabledForPackage4Worker");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}
