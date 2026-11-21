use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn OpenPrivateNamespaceA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _lpBoundaryDescriptor = api_try!(emu, call.arg_u32(emu, 0));
    let _lpAliasPrefix = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!OpenPrivateNamespaceA");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn OpenPrivateNamespaceW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 2);
    log_red!(emu, "kernel32!OpenPrivateNamespaceW");
    call.set_return(emu, helper::handler_create("OpenPrivateNamespaceW"));
}
