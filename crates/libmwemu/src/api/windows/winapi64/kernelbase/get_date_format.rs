use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetDateFormatA(emu: &mut Emu) {
    get_date_format(emu, "GetDateFormatA", false);
}

pub fn GetDateFormatW(emu: &mut Emu) {
    get_date_format(emu, "GetDateFormatW", true);
}

fn get_date_format(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 4));
    let capacity = api_try!(emu, call.arg_i32(emu, 5));
    let text = "11/01/2026";
    let result = strings::write_text(emu, buffer, capacity, text, wide);
    log_red!(emu, "kernelbase!{api} => '{}' result:{}", text, result);
    call.set_return(emu, result);
}
