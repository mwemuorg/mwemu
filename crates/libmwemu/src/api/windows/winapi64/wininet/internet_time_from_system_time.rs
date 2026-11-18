use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetTimeFromSystemTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "wininet!InternetTimeFromSystemTime arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InternetTimeFromSystemTimeA(emu: &mut emu::Emu) {
    internet_time_from_system_time(emu, "InternetTimeFromSystemTimeA", false);
}

fn internet_time_from_system_time(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let time_ptr = api_try!(emu, call.arg(emu, 0));
    let format = api_try!(emu, call.arg_u32(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "wininet!{api} format:{}", format);
    if buf_ptr != 0 {
        strings::write_string(emu, buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT", wide);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InternetTimeFromSystemTimeW(emu: &mut emu::Emu) {
    internet_time_from_system_time(emu, "InternetTimeFromSystemTimeW", true);
}
