use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetGetCookieExA(emu: &mut emu::Emu) {
    internet_get_cookie_ex(emu, "InternetGetCookieExA", false);
}

fn internet_get_cookie_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let size_ptr = api_try!(emu, call.arg(emu, 3));
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}'", url);
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    emu.set_last_error(259);
    call.set_return(emu, 0);
}

pub fn InternetGetCookieExW(emu: &mut emu::Emu) {
    internet_get_cookie_ex(emu, "InternetGetCookieExW", true);
}
