use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetSetCookieA(emu: &mut emu::Emu) {
    internet_set_cookie(emu, "InternetSetCookieA", false);
}

fn internet_set_cookie(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };
    let data = if data_ptr != 0 {
        strings::read_text(emu, data_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} url:'{}' name:'{}' data:'{}'",
        url,
        name,
        data
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn InternetSetCookieW(emu: &mut emu::Emu) {
    internet_set_cookie(emu, "InternetSetCookieW", true);
}
