use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateDirectoryExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _lpTemplateDirectory = api_try!(emu, call.arg_u32(emu, 0));
    let _lpNewDirectory = api_try!(emu, call.arg_u32(emu, 1));
    let _lpSecurityAttributes = api_try!(emu, call.arg_u32(emu, 2));

    log_red!(emu, "kernel32!CreateDirectoryExA");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn CreateDirectoryExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 3);
    log_red!(emu, "kernel32!CreateDirectoryExW");
    call.set_return(emu, helper::handler_create("CreateDirectoryExW"));
}
