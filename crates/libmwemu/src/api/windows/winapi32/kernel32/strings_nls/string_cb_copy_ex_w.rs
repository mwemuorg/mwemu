use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn StringCbCopyExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _pszDest = api_try!(emu, call.arg_u32(emu, 0));
    let _cbDest = api_try!(emu, call.arg_u32(emu, 1));
    let _pszSrc = api_try!(emu, call.arg_u32(emu, 2));
    let _ppszDestEnd = api_try!(emu, call.arg_u32(emu, 3));
    let _pcbRemaining = api_try!(emu, call.arg_u32(emu, 4));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 5));

    log_red!(emu, "kernel32!StringCbCopyExW");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}
