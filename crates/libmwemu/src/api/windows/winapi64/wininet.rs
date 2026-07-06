use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::emu;
use crate::serialization;
use crate::windows::constants;
//use crate::endpoint;
use crate::winapi::helper;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "InternetOpenA" => InternetOpenA(emu),
        "InternetOpenW" => InternetOpenW(emu),
        "InternetConnectA" => InternetConnectA(emu),
        "InternetConnectW" => InternetConnectW(emu),
        "HttpOpenRequestA" => HttpOpenRequestA(emu),
        "HttpOpenRequestW" => HttpOpenRequestW(emu),
        "InternetSetOptionA" => InternetSetOptionA(emu),
        "InternetSetOptionW" => InternetSetOptionW(emu),
        "HttpSendRequestA" => HttpSendRequestA(emu),
        "HttpSendRequestW" => HttpSendRequestW(emu),
        "InternetReadFile" => InternetReadFile(emu),
        "InternetReadFileExA" => InternetReadFileExA(emu),
        "InternetReadFileExW" => InternetReadFileExW(emu),
        "InternetErrorDlg" => InternetErrorDlg(emu),
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }

    String::new()
}

lazy_static! {
    static ref COUNT_RECEIVE: Mutex<u32> = Mutex::new(0);
}

pub fn InternetOpenA(emu: &mut emu::Emu) {
    let uagent_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let proxy_ptr = emu.regs().r8;
    let proxybypass_ptr = emu.regs().r9;
    let flags = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!InternetOpenA  cannot read flags");

    let mut uagent = "".to_string();
    let mut proxy = "".to_string();
    let mut proxy_bypass = "".to_string();

    if uagent_ptr != 0 {
        uagent = emu.maps.read_string(uagent_ptr);
    }
    if proxy_ptr != 0 {
        proxy = emu.maps.read_string(proxy_ptr);
    }
    if proxybypass_ptr != 0 {
        proxy_bypass = emu.maps.read_string(proxybypass_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetOpenA uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    /*
    if emu.cfg.endpoint {
        // endpoint mode
        if uagent_ptr != 0 && uagent != "" {
            endpoint::http_set_headers("User-Agent", &uagent);
        }
    }*/

    let uri = format!("InternetOpenA://{}", uagent);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub fn InternetOpenW(emu: &mut emu::Emu) {
    let uagent_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let proxy_ptr = emu.regs().r8;
    let proxybypass_ptr = emu.regs().r9;
    let flags = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!InternetOpenW  cannot read flags");

    let mut uagent = "".to_string();
    let mut proxy = "".to_string();
    let mut proxy_bypass = "".to_string();

    if uagent_ptr != 0 {
        uagent = emu.maps.read_wide_string(uagent_ptr);
    }
    if proxy_ptr != 0 {
        proxy = emu.maps.read_wide_string(proxy_ptr);
    }
    if proxybypass_ptr != 0 {
        proxy_bypass = emu.maps.read_wide_string(proxybypass_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetOpenW uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    /*
    if emu.cfg.endpoint {
        // endpoint mode
        if uagent_ptr != 0 && uagent != "" {
            endpoint::http_set_headers("User-Agent", &uagent.replace("\x00", ""));
        }
    }*/

    emu.regs_mut().rax = helper::handler_create("InternetOpenW://"); // internet handle
}

pub fn InternetConnectA(emu: &mut emu::Emu) {
    let internet_hndl = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8;
    let login_ptr = emu.regs().r9;
    let passw_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!InternetConnectA cannot read passw_ptr");
    let service = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("wininet!InternetConnectA cannot read service");
    let flags = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("wininet!InternetConnectA cannot read flags");
    let ctx = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("wininet!InternetConnectA cannot read ctx");

    let mut server = "".to_string();
    let mut login = "".to_string();
    let mut passw = "".to_string();

    if server_ptr != 0 {
        server = emu.maps.read_string(server_ptr);
    }
    if login_ptr != 0 {
        login = emu.maps.read_string(login_ptr);
    }
    if passw_ptr != 0 {
        passw = emu.maps.read_string(passw_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetConnectA host: {} port: {} login: {} passw: {}",
        server,
        port,
        login,
        passw
    );

    if !helper::handler_exist(internet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_serverport(&server, port as u16);
    }*/

    let uri = format!("InternetConnectA://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri); // connect handle
}

pub fn InternetConnectW(emu: &mut emu::Emu) {
    let internet_hndl = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8;
    let login_ptr = emu.regs().r9;
    let passw_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!InternetConnectW cannot read passw_ptr");
    let service = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("wininet!InternetConnectW cannot read service");
    let flags = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("wininet!InternetConnectW cannot read flags");
    let ctx = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("wininet!InternetConnectW cannot read ctx");

    let mut server = "".to_string();
    let mut login = "".to_string();
    let mut passw = "".to_string();

    if server_ptr != 0 {
        server = emu.maps.read_wide_string(server_ptr);
    }
    if login_ptr != 0 {
        login = emu.maps.read_wide_string(login_ptr);
    }
    if passw_ptr != 0 {
        passw = emu.maps.read_wide_string(passw_ptr);
    }

    log_red!(
        emu,
        "wininet!InternetConnectW host: {} port: {} login: {} passw: {}",
        server,
        port,
        login,
        passw
    );

    if !helper::handler_exist(internet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_serverport(&server.replace("\x00", ""), port as u16);
    }*/

    let uri = format!("InternetConnectW://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri); // connect handle
}

fn HttpOpenRequestA(emu: &mut emu::Emu) {
    let conn_hndl = emu.regs().rcx;
    let method_ptr = emu.regs().rdx;
    let path_ptr = emu.regs().r8;
    let version_ptr = emu.regs().r9;
    let referrer_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!HttpOpenRequestA cannot read referrer_ptr");
    let access_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("wininet!HttpOpenRequestA cannot read access_ptr");
    let flags = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("wininet!HttpOpenRequestA cannot read flags");
    let ctx = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("wininet!HttpOpenRequestA cannot read ctx");

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

    if flags & constants::INTERNET_FLAG_SECURE != 0 {
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

fn HttpOpenRequestW(emu: &mut emu::Emu) {
    let conn_hndl = emu.regs().rcx;
    let method_ptr = emu.regs().rdx;
    let path_ptr = emu.regs().r8;
    let version_ptr = emu.regs().r9;
    let referrer_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!HttpOpenRequestW cannot read referrer_ptr");
    let access_ptr = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("wininet!HttpOpenRequestW cannot read access_ptr");
    let flags = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("wininet!HttpOpenRequestW cannot read flags");
    let ctx = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("wininet!HttpOpenRequestW cannot read ctx");

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

fn InternetSetOptionA(emu: &mut emu::Emu) {
    let inet_hndl = emu.regs().rcx;
    let option = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let len = emu.regs().r9;

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = emu.maps.read_string(buffer);

    log_red!(
        emu,
        "wininet!InternetSetOptionA option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

fn InternetSetOptionW(emu: &mut emu::Emu) {
    let inet_hndl = emu.regs().rcx;
    let option = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let len = emu.regs().r9;

    let mut buffer_content = "".to_string();
    if buffer != 0 {
        buffer_content = emu.maps.read_string_of_bytes(buffer, len as usize);
    }
    let sbuff = emu.maps.read_wide_string(buffer);

    log_red!(
        emu,
        "wininet!InternetSetOptionW option: 0x{:x} buff: {{{}}} {}",
        option,
        buffer_content,
        sbuff
    );

    if !helper::handler_exist(inet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

fn HttpSendRequestA(emu: &mut emu::Emu) {
    let req_hndl = emu.regs().rcx;
    let hdrs_ptr = emu.regs().rdx;
    let hdrs_len = emu.regs().r8;
    let opt_ptr = emu.regs().r9;
    let opt_len = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!HttpSendRequestA cannot read opt_len");

    let hdrs = emu.maps.read_string(hdrs_ptr);
    let opt = emu.maps.read_string(opt_ptr);

    log_red!(emu, "wininet!HttpSendRequestA hdrs: {} opt: {}", hdrs, opt);

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

fn HttpSendRequestW(emu: &mut emu::Emu) {
    let req_hndl = emu.regs().rcx;
    let hdrs_ptr = emu.regs().rdx;
    let hdrs_len = emu.regs().r8;
    let opt_ptr = emu.regs().r9;
    let opt_len = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wininet!HttpSendRequestW cannot read opt_len");

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

fn InternetErrorDlg(emu: &mut emu::Emu) {
    let err = emu.regs().rcx;

    log_red!(emu, "wininet!InternetErrorDlg err: {}", err);

    emu.regs_mut().rax = 0;
}

fn InternetReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let bytes_to_read = emu.regs().r8;
    let bytes_read_ptr = emu.regs().r9;

    log_red!(
        emu,
        "wininet!InternetReadFile sz: {} buff: 0x{:x}",
        bytes_to_read,
        buff_ptr
    );

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    if emu.cfg.endpoint {
        /*
        let buff = endpoint::http_read_data();
        emu.maps.write_buffer(buff_ptr, &buff);
        emu.maps.write_dword(bytes_read_ptr, buff.len() as u32);
        */
    } else {
        let mut count = COUNT_RECEIVE.lock().unwrap();
        *count += 1;

        if *count < 3 {
            emu.maps.write_spaced_bytes(buff_ptr, "90 90 90 90");
            emu.maps.write_dword(bytes_read_ptr, bytes_to_read as u32);
        } else {
            emu.maps.write_dword(bytes_read_ptr, 0);
        }
    }

    emu.regs_mut().rax = 1; // true
}

fn InternetReadFileExA(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let ctx = emu.regs().r9;

    log_red!(emu, "wininet!InternetReadFileExA buff: 0x{:x}", buff_ptr);

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}

fn InternetReadFileExW(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let buff_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let ctx = emu.regs().r9;

    log_red!(emu, "wininet!InternetReadFileExW buff: 0x{:x}", buff_ptr);

    if !helper::handler_exist(file_hndl) {
        log::trace!("\tinvalid handle.");
    }

    emu.regs_mut().rax = 1; // true
}
