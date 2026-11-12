use crate::emu;
use crate::winapi::common::memory::Call;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::network::*;

pub(super) fn http_open_request_a(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestA", false);
}

fn http_open_request(emu: &mut emu::Emu, api: &str, wide: bool) {
    let conn_hndl = WinApiAbi::arg(emu, 0);
    let method_ptr = WinApiAbi::arg(emu, 1);
    let path_ptr = WinApiAbi::arg(emu, 2);
    let version_ptr = WinApiAbi::arg(emu, 3);
    let referrer_ptr = WinApiAbi::arg(emu, 4);
    let access_ptr = WinApiAbi::arg(emu, 5);
    let flags = WinApiAbi::arg(emu, 6);
    let _ctx = WinApiAbi::arg(emu, 7) as u32;

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

    WinApiAbi::stdcall_ret(emu, 8);

    let uri = format!("{api}://{}", path);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub(super) fn http_open_request_w(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestW", true);
}

pub(super) fn internet_set_option_a(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionA", false);
}

fn internet_set_option(emu: &mut emu::Emu, api: &str, wide: bool) {
    let inet_hndl = WinApiAbi::arg(emu, 0);
    let option = WinApiAbi::arg(emu, 1) as u32;
    let buffer = WinApiAbi::arg(emu, 2);
    let len = WinApiAbi::arg(emu, 3) as u32;

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

    WinApiAbi::stdcall_ret(emu, 4);

    emu.regs_mut().rax = 1;
}

pub(super) fn internet_set_option_w(emu: &mut emu::Emu) {
    internet_set_option(emu, "InternetSetOptionW", true);
}

pub(super) fn http_send_request_a(emu: &mut emu::Emu) {
    http_send_request(emu, "HttpSendRequestA", false);
}

fn http_send_request(emu: &mut emu::Emu, api: &str, wide: bool) {
    let req_hndl = WinApiAbi::arg(emu, 0);
    let hdrs_ptr = WinApiAbi::arg(emu, 1);
    let _hdrs_len = WinApiAbi::arg(emu, 2) as u32;
    let opt_ptr = WinApiAbi::arg(emu, 3);
    let _opt_len = WinApiAbi::arg(emu, 4) as u32;

    let hdrs = strings::read_text(emu, hdrs_ptr, wide);
    let opt = strings::read_text(emu, opt_ptr, wide);

    log_red!(emu, "wininet!{api} hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    WinApiAbi::stdcall_ret(emu, 5);

    emu.regs_mut().rax = 1;
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
    let buffer = Call::Api.arg(emu, 2);
    let length = Call::Api.arg(emu, 3);
    log_red!(
        emu,
        "wininet!{api} buffer:0x{:x} length:0x{:x}",
        buffer,
        length
    );
    Call::Api.finish(emu, 1, 5);
}
