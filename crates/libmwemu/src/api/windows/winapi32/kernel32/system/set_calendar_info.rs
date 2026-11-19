use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn SetCalendarInfoA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _Locale = api_try!(emu, call.arg_u32(emu, 0));
    let _Calendar = api_try!(emu, call.arg_u32(emu, 1));
    let _CalType = api_try!(emu, call.arg_u32(emu, 2));
    let _lpCalData = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(emu, "kernel32!SetCalendarInfoA");

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 4);
}

pub fn SetCalendarInfoW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "kernel32!SetCalendarInfoW '{}'  0x{:x}", s, arg2);
    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}
