use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetShortPathNameA(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameA", false);
}

fn get_short_path_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpszLongPath = api_try!(emu, call.arg_u32(emu, 0));
    let lpszShortPath = api_try!(emu, call.arg_u32(emu, 1));
    let _cchBuffer = api_try!(emu, call.arg_u32(emu, 2));

    let lpszLongPath = strings::read_text(emu, lpszLongPath as u64, wide);
    let lpszShortPath = strings::read_text(emu, lpszShortPath as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpszLongPath, lpszShortPath);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn GetShortPathNameW(emu: &mut emu::Emu) {
    get_short_path_name(emu, "GetShortPathNameW", true);
}
