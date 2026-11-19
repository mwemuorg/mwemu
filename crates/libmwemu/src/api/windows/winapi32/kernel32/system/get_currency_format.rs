use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn GetCurrencyFormatA(emu: &mut emu::Emu) {
    get_currency_format(emu, "GetCurrencyFormatA", false);
}

fn get_currency_format(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _Locale = api_try!(emu, call.arg_u32(emu, 0));
    let _dwFlags = api_try!(emu, call.arg_u32(emu, 1));
    let _lpValue = api_try!(emu, call.arg_u32(emu, 2));
    let _lpFormat = api_try!(emu, call.arg_u32(emu, 3));
    let lpCurrencyStr = api_try!(emu, call.arg_u32(emu, 4));
    let _cchCurrency = api_try!(emu, call.arg_u32(emu, 5));

    let lpCurrencyStr = strings::read_text(emu, lpCurrencyStr as u64, wide);

    log_red!(emu, "kernel32!{api} {}", lpCurrencyStr);

    call.set_return(emu, ERROR_SUCCESS);
    call.cleanup_args(emu, 6);
}

pub fn GetCurrencyFormatW(emu: &mut emu::Emu) {
    get_currency_format(emu, "GetCurrencyFormatW", true);
}
