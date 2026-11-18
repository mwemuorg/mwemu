use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn LZCopy(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hfSource = api_try!(emu, call.arg_u32(emu, 0));
    let _hfDest = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!LZCopy");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}
