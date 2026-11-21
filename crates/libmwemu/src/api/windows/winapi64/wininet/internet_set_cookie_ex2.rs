use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetSetCookieEx2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let cookie_ptr = api_try!(emu, call.arg(emu, 1));
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    let cookie = if cookie_ptr != 0 {
        emu.maps.read_string(cookie_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetSetCookieEx2 url:'{}' cookie:'{}'",
        url,
        cookie
    );
    emu.set_last_error(0);
    call.set_return(emu, 0); // COOKIE_STATE_ACCEPT
}
