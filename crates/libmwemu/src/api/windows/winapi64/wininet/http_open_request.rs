use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::network::*;

pub(super) fn HttpOpenRequestA(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestA", false);
}

pub(super) fn HttpOpenRequestW(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestW", true);
}

fn http_open_request(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let conn_hndl = api_try!(emu, call.arg(emu, 0));
    let method_ptr = api_try!(emu, call.arg(emu, 1));
    let path_ptr = api_try!(emu, call.arg(emu, 2));
    let version_ptr = api_try!(emu, call.arg(emu, 3));
    let referrer_ptr = api_try!(emu, call.arg(emu, 4));
    let access_ptr = api_try!(emu, call.arg(emu, 5));
    let flags = api_try!(emu, call.arg(emu, 6));
    let ctx = api_try!(emu, call.arg(emu, 7));
    let mut method = "".to_string();
    let mut path = "".to_string();
    let mut version = "".to_string();
    let mut referrer = "".to_string();
    let mut access = "".to_string();
    if method_ptr != 0 {
        method = strings::read_text(emu, method_ptr, wide);
    }
    if path_ptr != 0 {
        path = strings::read_text(emu, path_ptr, wide);
    }
    if version_ptr != 0 {
        version = strings::read_text(emu, version_ptr, wide);
    }
    if referrer_ptr != 0 {
        referrer = strings::read_text(emu, referrer_ptr, wide);
    }
    if access_ptr != 0 {
        access = strings::read_text(emu, access_ptr, wide);
    }
    log_red!(
        emu,
        "wininet!{api} method: {} path: {} ver: {} ref: {} access: {}",
        method,
        path,
        version,
        referrer,
        access
    );
    if !helper::handler_exist(conn_hndl) {
        log::trace!("\tinvalid handle.");
    }
    if wide {
    } else {
        if flags & INTERNET_FLAG_SECURE != 0 {
            log::trace!("\tssl communication.");
        }
    }

    let uri = format!("{api}://{}", path);
    call.set_return(emu, helper::handler_create(&uri));
}
