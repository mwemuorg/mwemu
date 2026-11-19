use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn MulDiv(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _nNumber = api_try!(emu, call.arg_u32(emu, 0));
    let _nNumerator = api_try!(emu, call.arg_u32(emu, 1));
    let _nDenominator = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!MulDiv");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}
