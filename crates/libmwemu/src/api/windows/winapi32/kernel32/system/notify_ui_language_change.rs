use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn NotifyUILanguageChange(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 0));
    let _pcwstrNewLanguage = api_try!(emu, call.arg_u32(emu, 1));
    let _pcwstrPreviousLanguage = api_try!(emu, call.arg_u32(emu, 2));
    let _dwReserved = api_try!(emu, call.arg_u32(emu, 3));
    let _pdwStatusRtrn = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(emu, "kernel32!NotifyUILanguageChange");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 5);
}
