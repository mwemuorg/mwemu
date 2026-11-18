use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetDurationFormat(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _Locale = api_try!(emu, call.arg_u32(emu, 0));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 1));
    let _lpDuration = api_try!(emu, call.arg_u32(emu, 2));
    let _ullDuration = api_try!(emu, call.arg_u32(emu, 3));
    let _lpFormat = api_try!(emu, call.arg_u32(emu, 5));
    let _lpDurationStr = api_try!(emu, call.arg_u32(emu, 6));
    let _cchDuration = api_try!(emu, call.arg_u32(emu, 7));

    log_red!(emu, "kernel32!GetDurationFormat");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 8);
}
