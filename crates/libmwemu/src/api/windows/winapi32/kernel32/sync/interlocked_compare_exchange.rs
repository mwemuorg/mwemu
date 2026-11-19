use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn InterlockedCompareExchange(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _Destination = api_try!(emu, call.arg_u32(emu, 0));
    let _Exchange = api_try!(emu, call.arg_u32(emu, 1));
    let _Comperand = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!InterlockedCompareExchange");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}
