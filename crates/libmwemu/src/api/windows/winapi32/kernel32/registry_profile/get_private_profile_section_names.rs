use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetPrivateProfileSectionNamesA(emu: &mut emu::Emu) {
    get_private_profile_section_names(emu, "GetPrivateProfileSectionNamesA", false);
}

fn get_private_profile_section_names(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _lpszReturnBuffer = api_try!(emu, call.arg_u32(emu, 0));
    let _nSize = api_try!(emu, call.arg_u32(emu, 1));
    let lpFileName = api_try!(emu, call.arg_u32(emu, 2));

    let lpFileName = strings::read_text(emu, lpFileName as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpFileName);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 3);
}

pub fn GetPrivateProfileSectionNamesW(emu: &mut emu::Emu) {
    get_private_profile_section_names(emu, "GetPrivateProfileSectionNamesW", true);
}
