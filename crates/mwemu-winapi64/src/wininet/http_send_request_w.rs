use crate::emu;
use crate::winapi::helper;

pub(super) fn HttpSendRequestW(emu: &mut emu::Emu) {
    let req_hndl = emu.regs().rcx;
    let hdrs_ptr = emu.regs().rdx;
    let hdrs_len = emu.regs().r8;
    let opt_ptr = emu.regs().r9;
    let opt_len = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let hdrs = emu.maps.read_wide_string(hdrs_ptr);
    let opt = emu.maps.read_wide_string(opt_ptr);

    log_red!(emu, "wininet!HttpSendRequestW hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_headers_str(&hdrs.replace("\x00", ""));
        endpoint::http_send_request();
    }*/

    emu.regs_mut().rax = 1; // true
}
