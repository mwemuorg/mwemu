use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BaseDllInitialize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hinstDLL = api_try!(emu, call.arg_u32(emu, 0));
    let _fdwReason = api_try!(emu, call.arg_u32(emu, 1));
    let _lpReserved = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!BaseDllInitialize");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}
