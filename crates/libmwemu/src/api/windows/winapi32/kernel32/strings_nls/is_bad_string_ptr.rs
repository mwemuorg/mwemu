use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn IsBadStringPtrA(emu: &mut emu::Emu) {
    is_bad_string_ptr(emu, "IsBadStringPtrA");
}

fn is_bad_string_ptr(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let _lpsz = api_try!(emu, call.arg_u32(emu, 0));
    let _ucchMax = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!{api}");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 2);
}

pub fn IsBadStringPtrW(emu: &mut emu::Emu) {
    is_bad_string_ptr(emu, "IsBadStringPtrW");
}
