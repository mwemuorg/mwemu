use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetGetCookieEx2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let size_ptr = api_try!(emu, call.arg(emu, 3));
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!InternetGetCookieEx2 url:'{}'", url);
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    emu.set_last_error(259);
    call.set_return(emu, 0);
}
