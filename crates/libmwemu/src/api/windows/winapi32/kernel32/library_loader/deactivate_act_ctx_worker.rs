use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn DeactivateActCtxWorker(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _int = api_try!(emu, call.arg_u32(emu, 0));
    let _ulCookie = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!DeactivateActCtxWorker");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}
