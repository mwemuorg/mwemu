use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn WriteProfileStringA(emu: &mut emu::Emu) {
    write_profile_string(emu, "WriteProfileStringA", false);
}

fn write_profile_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpAppName = api_try!(emu, call.arg_u32(emu, 0));
    let lpKeyName = api_try!(emu, call.arg_u32(emu, 1));
    let lpString = api_try!(emu, call.arg_u32(emu, 2));

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpKeyName = strings::read_text(emu, lpKeyName as u64, wide);
    let lpString = strings::read_text(emu, lpString as u64, wide);

    log_red!(
        emu,
        "kernel32!{api} {} {} {}",
        lpAppName,
        lpKeyName,
        lpString
    );

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn WriteProfileStringW(emu: &mut emu::Emu) {
    write_profile_string(emu, "WriteProfileStringW", true);
}
