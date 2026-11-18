use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn InternetCanonicalizeUrlA(emu: &mut emu::Emu) {
    internet_canonicalize_url(emu, "InternetCanonicalizeUrlA", false);
}

fn internet_canonicalize_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let url_ptr = api_try!(emu, call.arg(emu, 0));
    let out_ptr = api_try!(emu, call.arg(emu, 1));
    let out_len_ptr = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}' flags:0x{:x}", url, flags);
    if out_ptr != 0 {
        strings::write_string(emu, out_ptr, &url, wide);
    }
    if out_len_ptr != 0 {
        emu.maps.write_dword(out_len_ptr, url.len() as u32);
    }
    call.set_return(emu, 1);
}

pub fn InternetCanonicalizeUrlW(emu: &mut emu::Emu) {
    internet_canonicalize_url(emu, "InternetCanonicalizeUrlW", true);
}
