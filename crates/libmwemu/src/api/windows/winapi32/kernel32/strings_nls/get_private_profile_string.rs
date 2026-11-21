use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileStringA(emu: &mut emu::Emu) {
    get_private_profile_string(emu, "GetPrivateProfileStringA", false);
}

fn get_private_profile_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpAppName = api_try!(emu, call.arg_u32(emu, 0));
    let lpKeyName = api_try!(emu, call.arg_u32(emu, 1));
    let _lpDefault = api_try!(emu, call.arg_u32(emu, 2));
    let lpReturnedString = api_try!(emu, call.arg_u32(emu, 3));
    let _nSize = api_try!(emu, call.arg_u32(emu, 4));
    let lpFileName = api_try!(emu, call.arg_u32(emu, 5));

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);
    let lpReturnedString = strings::read_text(emu, lpReturnedString as u64, wide);
    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {} {}",
        lpAppName,
        lpKeyName,
        lpReturnedString,
        lpFileName
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn GetPrivateProfileStringW(emu: &mut emu::Emu) {
    get_private_profile_string(emu, "GetPrivateProfileStringW", true);
}
