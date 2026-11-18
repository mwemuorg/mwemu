use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetNamedPipeClientProcessId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _Pipe = api_try!(emu, call.arg_u32(emu, 0));
    let _ClientProcessId = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!GetNamedPipeClientProcessId");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}
