use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileIntA(emu: &mut emu::Emu) {
    get_private_profile_int(emu, "GetPrivateProfileIntA", false);
}

fn get_private_profile_int(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpAppName = api_try!(emu, call.arg_u32(emu, 0));
    let lpKeyName = api_try!(emu, call.arg_u32(emu, 1));
    let _nDefault = api_try!(emu, call.arg_u32(emu, 2));
    let lpFileName = api_try!(emu, call.arg_u32(emu, 3));

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);
    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {}",
        lpAppName,
        lpKeyName,
        lpFileName
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn GetPrivateProfileIntW(emu: &mut emu::Emu) {
    get_private_profile_int(emu, "GetPrivateProfileIntW", true);
}
