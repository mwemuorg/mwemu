use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetFileShortNameA(emu: &mut emu::Emu) {
    set_file_short_name(emu, "SetFileShortNameA", false);
}

fn set_file_short_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _hFile = api_try!(emu, call.arg_u32(emu, 0));
    let lpShortName = api_try!(emu, call.arg_u32(emu, 1));

    let lpShortName = strings::read_text(emu, lpShortName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpShortName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn SetFileShortNameW(emu: &mut emu::Emu) {
    set_file_short_name(emu, "SetFileShortNameW", true);
}
