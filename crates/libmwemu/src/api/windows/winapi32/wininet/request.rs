use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::network::*;

pub(super) fn http_open_request_a(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestA", false);
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
    let _ctx = api_try!(emu, call.arg_u32(emu, 7));

    let method = strings::read_text(emu, method_ptr, wide);
    let path = strings::read_text(emu, path_ptr, wide);
    let version = strings::read_text(emu, version_ptr, wide);
    let referrer = strings::read_text(emu, referrer_ptr, wide);
    let access = strings::read_text(emu, access_ptr, wide);

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

    if flags & INTERNET_FLAG_SECURE != 0 {
        log::trace!("\tssl communication.");
    }

    call.cleanup_args(emu, 8);

    let uri = format!("{api}://{}", path);
    call.set_return(emu, helper::handler_create(&uri));
}

pub(super) fn http_open_request_w(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestW", true);
}

pub(super) fn internet_set_option_a(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionA", false);
}

fn internet_set_option(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let inet_hndl = api_try!(emu, call.arg(emu, 0));
    let option = api_try!(emu, call.arg_u32(emu, 1));
    let buffer = api_try!(emu, call.arg(emu, 2));
    let len = api_try!(emu, call.arg_u32(emu, 3));

    let mut buffer_content = String::new();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = strings::read_text(emu, buffer, wide);

    log_red!(
        emu,
        "wininet!{api} option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    call.cleanup_args(emu, 4);

    call.set_return(emu, 1);
}

pub(super) fn internet_set_option_w(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionW", true);
}

pub(super) fn http_send_request_a(emu: &mut emu::Emu) {
    http_send_request(emu, "HttpSendRequestA", false);
}

fn http_send_request(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let req_hndl = api_try!(emu, call.arg(emu, 0));
    let hdrs_ptr = api_try!(emu, call.arg(emu, 1));
    let _hdrs_len = api_try!(emu, call.arg_u32(emu, 2));
    let opt_ptr = api_try!(emu, call.arg(emu, 3));
    let _opt_len = api_try!(emu, call.arg_u32(emu, 4));

    let hdrs = strings::read_text(emu, hdrs_ptr, wide);
    let opt = strings::read_text(emu, opt_ptr, wide);

    log_red!(emu, "wininet!{api} hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    call.cleanup_args(emu, 5);

    call.set_return(emu, 1);
}

pub(super) fn http_send_request_w(emu: &mut emu::Emu) {
    http_send_request(emu, "HttpSendRequestW", true);
}

pub fn HttpQueryInfoA(emu: &mut emu::Emu) {
    http_query_info(emu, "HttpQueryInfoA");
}

pub fn HttpQueryInfoW(emu: &mut emu::Emu) {
    http_query_info(emu, "HttpQueryInfoW");
}

fn http_query_info(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let buffer = api_try!(emu, call.arg(emu, 2));
    let length = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "wininet!{api} buffer:0x{:x} length:0x{:x}",
        buffer,
        length
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
