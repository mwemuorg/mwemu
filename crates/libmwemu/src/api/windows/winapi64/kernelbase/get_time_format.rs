use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetTimeFormatA(emu: &mut Emu) {
    get_time_format(emu, "GetTimeFormatA", false);
}

pub fn GetTimeFormatW(emu: &mut Emu) {
    get_time_format(emu, "GetTimeFormatW", true);
}

fn get_time_format(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 4));
    let capacity = api_try!(emu, call.arg_i32(emu, 5));
    let text = "12:00:00";
    let result = strings::write_text(emu, buffer, capacity, text, wide);
    log_red!(emu, "kernelbase!{api} => '{}' result:{}", text, result);
    call.set_return(emu, result);
}
