use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn InternetOpenUrlA(emu: &mut emu::Emu) {
    internet_open_url(emu, "InternetOpenUrlA", false);
}

pub fn InternetOpenUrlW(emu: &mut emu::Emu) {
    internet_open_url(emu, "InternetOpenUrlW", true);
}

fn internet_open_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let inet_handle = api_try!(emu, call.arg(emu, 0));
    let url_ptr = api_try!(emu, call.arg(emu, 1));
    let headers_ptr = api_try!(emu, call.arg(emu, 2));
    let headers_len = api_try!(emu, call.arg_u32(emu, 3));
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} inet:0x{:x} url:'{}'", inet_handle, url);
    let uri = format!("{api}://{}", url);
    let handle = helper::handler_create(&uri);
    call.set_return(emu, handle);
}
