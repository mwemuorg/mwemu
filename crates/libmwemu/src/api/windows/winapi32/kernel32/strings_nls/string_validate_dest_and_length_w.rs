use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn StringValidateDestAndLengthW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _pszDest = api_try!(emu, call.arg_u32(emu, 0));
    let _cchDest = api_try!(emu, call.arg_u32(emu, 1));
    let _pcchDestLength = api_try!(emu, call.arg_u32(emu, 2));
    let _cchMax = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(emu, "kernel32!StringValidateDestAndLengthW");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}
