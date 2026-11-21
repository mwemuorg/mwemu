use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetCreateUrlA(emu: &mut emu::Emu) {
    internet_create_url(emu, "InternetCreateUrlA", false);
}

fn internet_create_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}'", url);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InternetCreateUrlW(emu: &mut emu::Emu) {
    internet_create_url(emu, "InternetCreateUrlW", true);
}
