use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetCheckConnectionA(emu: &mut emu::Emu) {
    internet_check_connection(emu, "InternetCheckConnectionA", false);
}

fn internet_check_connection(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}' flags:0x{:x}", url, flags);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InternetCheckConnectionW(emu: &mut emu::Emu) {
    internet_check_connection(emu, "InternetCheckConnectionW", true);
}
