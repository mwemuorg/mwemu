use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn HttpSendRequestA(emu: &mut emu::Emu) {
    http_send_request(emu, "HttpSendRequestA", false);
}

pub(super) fn HttpSendRequestW(emu: &mut emu::Emu) {
    http_send_request(emu, "HttpSendRequestW", true);
}

fn http_send_request(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let req_hndl = api_try!(emu, call.arg(emu, 0));
    let hdrs_ptr = api_try!(emu, call.arg(emu, 1));
    let hdrs_len = api_try!(emu, call.arg(emu, 2));
    let opt_ptr = api_try!(emu, call.arg(emu, 3));
    let opt_len = api_try!(emu, call.arg(emu, 4));

    let hdrs = strings::read_text(emu, hdrs_ptr, wide);
    let opt = strings::read_text(emu, opt_ptr, wide);

    log_red!(emu, "wininet!{api} hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_headers_str(&hdrs);
        endpoint::http_send_request();
    }*/

    call.set_return(emu, 1); // true
}
