use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::network::*;

pub(super) fn HttpOpenRequestA(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestA", false);
}

pub(super) fn HttpOpenRequestW(emu: &mut emu::Emu) {
    http_open_request(emu, "HttpOpenRequestW", true);
}

fn http_open_request(emu: &mut emu::Emu, api: &str, wide: bool) {
    let conn_hndl = emu.regs().rcx;
    let method_ptr = emu.regs().rdx;
    let path_ptr = emu.regs().r8;
    let version_ptr = emu.regs().r9;
    let referrer_ptr = WinApiAbi::arg(emu, 4);
    let access_ptr = WinApiAbi::arg(emu, 5);
    let flags = WinApiAbi::arg(emu, 6);
    let ctx = WinApiAbi::arg(emu, 7);
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
    emu.regs_mut().rax = helper::handler_create(&uri);
}
