use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::network::*;

pub(super) fn HttpOpenRequestA(emu: &mut emu::Emu) {
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
        method = emu.maps.read_string(method_ptr);
    }
    if path_ptr != 0 {
        path = emu.maps.read_string(path_ptr);
    }
    if version_ptr != 0 {
        version = emu.maps.read_string(version_ptr);
    }
    if referrer_ptr != 0 {
        referrer = emu.maps.read_string(referrer_ptr);
    }
    if access_ptr != 0 {
        access = emu.maps.read_string(access_ptr);
    }

    log_red!(
        emu,
        "wininet!HttpOpenRequestA method: {} path: {} ver: {} ref: {} access: {}",
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

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_path(&path);
        if flags & constants::INTERNET_FLAG_SECURE == 1 {
            endpoint::http_set_ssl();
        }

        if method_ptr != 0 {
            if method == "" {
                endpoint::http_set_method("get");
            } else {
                endpoint::http_set_method(&method);
            }
        } else {
            endpoint::http_set_method("get");
        }
    }*/

    let uri = format!("HttpOpenRequestA://{}", path);
    emu.regs_mut().rax = helper::handler_create(&uri); // request handle
}

pub(super) fn HttpOpenRequestW(emu: &mut emu::Emu) {
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
        method = emu.maps.read_wide_string(method_ptr);
    }
    if path_ptr != 0 {
        path = emu.maps.read_wide_string(path_ptr);
    }
    if version_ptr != 0 {
        version = emu.maps.read_wide_string(version_ptr);
    }
    if referrer_ptr != 0 {
        referrer = emu.maps.read_wide_string(referrer_ptr);
    }
    if access_ptr != 0 {
        access = emu.maps.read_wide_string(access_ptr);
    }

    log_red!(
        emu,
        "wininet!HttpOpenRequestW method: {} path: {} ver: {} ref: {} access: {}",
        method,
        path,
        version,
        referrer,
        access
    );

    if !helper::handler_exist(conn_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_path(&path);
        if flags & constants::INTERNET_FLAG_SECURE == 1 {
            endpoint::http_set_ssl();
        }

        if method_ptr != 0 {
            if method.len() < 3 {
                endpoint::http_set_method("get");
            } else {
                endpoint::http_set_method(&method.replace("\x00", ""));
            }
        } else {
            endpoint::http_set_method("get");
        }
    }*/

    let uri = format!("HttpOpenRequestW://{}", path);
    emu.regs_mut().rax = helper::handler_create(&uri); // request handle
}
