use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn WritePrivateProfileStructA(emu: &mut emu::Emu) {
    write_private_profile_struct(emu, "WritePrivateProfileStructA", false);
}

fn write_private_profile_struct(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _lpszSection = api_try!(emu, call.arg_u32(emu, 0));
    let _lpszKey = api_try!(emu, call.arg_u32(emu, 1));
    let lpStruct = api_try!(emu, call.arg_u32(emu, 2));
    let uSizeStruct = api_try!(emu, call.arg_u32(emu, 3));
    let _szFile = api_try!(emu, call.arg_u32(emu, 4));

    let lpStruct = strings::read_text(emu, lpStruct as u64, wide);
    let uSizeStruct = strings::read_text(emu, uSizeStruct as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpStruct, uSizeStruct);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}

pub fn WritePrivateProfileStructW(emu: &mut emu::Emu) {
    write_private_profile_struct(emu, "WritePrivateProfileStructW", true);
}
