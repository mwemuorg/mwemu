use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetExpandedNameA(emu: &mut emu::Emu) {
    get_expanded_name(emu, "GetExpandedNameA");
}

fn get_expanded_name(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _lpszSource = api_try!(emu, call.arg_u32(emu, 0));
    let _lpszBuffer = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn GetExpandedNameW(emu: &mut emu::Emu) {
    get_expanded_name(emu, "GetExpandedNameW");
}
