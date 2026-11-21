use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetNumberFormatA(emu: &mut emu::Emu) {
    get_number_format(emu, "GetNumberFormatA", false);
}

fn get_number_format(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _Locale = api_try!(emu, call.arg_u32(emu, 0));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 1));
    let _lpValue = api_try!(emu, call.arg_u32(emu, 2));
    let _lpFormat = api_try!(emu, call.arg_u32(emu, 3));
    let lpNumberStr = api_try!(emu, call.arg_u32(emu, 4));
    let _cchNumber = api_try!(emu, call.arg_u32(emu, 5));

    let lpNumberStr = strings::read_text(emu, lpNumberStr as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpNumberStr);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn GetNumberFormatW(emu: &mut emu::Emu) {
    get_number_format(emu, "GetNumberFormatW", true);
}
