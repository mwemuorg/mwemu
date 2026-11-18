use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn CreateFileMappingNumaA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hFile = api_try!(emu, call.arg_u32(emu, 0));
    let _lpFileMappingAttributes = api_try!(emu, call.arg_u32(emu, 1));
    let _flProtect = api_try!(emu, call.arg_u32(emu, 2));
    let _dwMaximumSizeHigh = api_try!(emu, call.arg_u32(emu, 3));
    let _dwMaximumSizeLow = api_try!(emu, call.arg_u32(emu, 4));
    let lpName = api_try!(emu, call.arg_u32(emu, 5));
    let _nndPreferred = api_try!(emu, call.arg_u32(emu, 6));

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateFileMappingNumaA {}", lpName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 7);
}

pub fn CreateFileMappingNumaW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    call.cleanup_args(emu, 7);
    log_red!(emu, "kernel32!CreateFileMappingNumaW");
    call.set_return(emu, helper::handler_create("CreateFileMappingNumaW"));
}
