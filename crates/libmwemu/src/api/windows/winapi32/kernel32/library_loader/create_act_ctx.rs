use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateActCtxA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _pActCtx = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernel32!CreateActCtxA");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 1);
}

pub fn CreateActCtxW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 1);
    log_red!(emu, "kernel32!CreateActCtxW");
    call.set_return(emu, helper::handler_create("CreateActCtxW"));
}
