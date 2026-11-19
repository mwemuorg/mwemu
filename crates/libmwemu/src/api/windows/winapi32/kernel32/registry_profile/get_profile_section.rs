use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetProfileSectionA(emu: &mut emu::Emu) {
    get_profile_section(emu, "GetProfileSectionA", false);
}

fn get_profile_section(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpAppName = api_try!(emu, call.arg_u32(emu, 0));
    let lpReturnedString = api_try!(emu, call.arg_u32(emu, 1));
    let _nSize = api_try!(emu, call.arg_u32(emu, 2));

    let lpAppName = strings::read_text(emu, lpAppName as u64, wide);
    let lpReturnedString = strings::read_text(emu, lpReturnedString as u64, wide);

    log_red!(emu, "kernel32!{api} {} {}", lpAppName, lpReturnedString);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn GetProfileSectionW(emu: &mut emu::Emu) {
    get_profile_section(emu, "GetProfileSectionW", true);
}
