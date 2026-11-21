use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn NtVdm64CreateProcessInternalW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _a1 = api_try!(emu, call.arg_u32(emu, 0));
    let _a2 = api_try!(emu, call.arg_u32(emu, 1));
    let _cchDest = api_try!(emu, call.arg_u32(emu, 2));
    let _a4 = api_try!(emu, call.arg_u32(emu, 3));
    let _a5 = api_try!(emu, call.arg_u32(emu, 4));
    let _a6 = api_try!(emu, call.arg_u32(emu, 5));
    let _a7 = api_try!(emu, call.arg_u32(emu, 6));
    let _a8 = api_try!(emu, call.arg_u32(emu, 7));
    let _a9 = api_try!(emu, call.arg_u32(emu, 8));
    let _a10 = api_try!(emu, call.arg_u32(emu, 9));
    let _a11 = api_try!(emu, call.arg_u32(emu, 10));
    let _a12 = api_try!(emu, call.arg_u32(emu, 11));

    log_red!(emu, "kernel32!NtVdm64CreateProcessInternalW");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 12);
}
