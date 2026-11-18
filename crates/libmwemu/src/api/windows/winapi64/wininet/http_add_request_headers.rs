use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn HttpAddRequestHeadersA(emu: &mut emu::Emu) {
    http_add_request_headers(emu, "HttpAddRequestHeadersA", false);
}

fn http_add_request_headers(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let headers_ptr = api_try!(emu, call.arg(emu, 1));
    let headers_len = api_try!(emu, call.arg_i32(emu, 2));
    let modifiers = api_try!(emu, call.arg_u32(emu, 3));
    let hdrs = if headers_ptr != 0 {
        strings::read_text(emu, headers_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} hdrs:'{}' mod:0x{:x}",
        handle,
        hdrs,
        modifiers
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn HttpAddRequestHeadersW(emu: &mut emu::Emu) {
    http_add_request_headers(emu, "HttpAddRequestHeadersW", true);
}
