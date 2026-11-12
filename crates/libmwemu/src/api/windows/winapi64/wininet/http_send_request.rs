use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn HttpSendRequestA(emu: &mut emu::Emu) {
    http_send_request(emu, "HttpSendRequestA", false);
}

pub(super) fn HttpSendRequestW(emu: &mut emu::Emu) {
    http_send_request(emu, "HttpSendRequestW", true);
}

fn http_send_request(emu: &mut emu::Emu, api: &str, wide: bool) {
    let req_hndl = emu.regs().rcx;
    let hdrs_ptr = emu.regs().rdx;
    let hdrs_len = emu.regs().r8;
    let opt_ptr = emu.regs().r9;
    let opt_len = WinApiAbi::arg(emu, 4);

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

    emu.regs_mut().rax = 1; // true
}
