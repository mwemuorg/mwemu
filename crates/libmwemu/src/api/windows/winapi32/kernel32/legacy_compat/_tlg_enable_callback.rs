use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn _tlgEnableCallback(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _pSourceId = api_try!(emu, call.arg_u32(emu, 0));
    let _callbackType = api_try!(emu, call.arg_u32(emu, 1));
    let _level = api_try!(emu, call.arg_u32(emu, 2));
    let _keywordAny = api_try!(emu, call.arg_u32(emu, 3));
    let _keywordAll = api_try!(emu, call.arg_u32(emu, 4));
    let _pFilterData = api_try!(emu, call.arg_u32(emu, 5));
    let _pCallbackContext = api_try!(emu, call.arg_u32(emu, 6));

    log_red!(emu, "kernel32!_tlgEnableCallback");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 7);
}
