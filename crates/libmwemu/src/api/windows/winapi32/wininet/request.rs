use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

use super::{
    pop_stack32, read_ansi_string_if_present, read_stack_dword, read_wide_string_if_present,
};

pub(super) fn http_open_request_a(emu: &mut emu::Emu) {
    let conn_hndl = read_stack_dword(emu, 0, "wininet!HttpOpenRequestA cannot read hndl") as u64;
    let method_ptr =
        read_stack_dword(emu, 4, "wininet!HttpOpenRequestA cannot read method_ptr") as u64;
    let path_ptr = read_stack_dword(emu, 8, "wininet!HttpOpenRequestA cannot read path_ptr") as u64;
    let version_ptr =
        read_stack_dword(emu, 12, "wininet!HttpOpenRequestA cannot read version_ptr") as u64;
    let referrer_ptr =
        read_stack_dword(emu, 16, "wininet!HttpOpenRequestA cannot read referrer_ptr") as u64;
    let access_ptr =
        read_stack_dword(emu, 20, "wininet!HttpOpenRequestA cannot read access_ptr") as u64;
    let flags = read_stack_dword(emu, 24, "wininet!HttpOpenRequestA cannot read flags") as u64;
    let _ctx = read_stack_dword(emu, 28, "wininet!HttpOpenRequestA cannot read ctx");

    let method = read_ansi_string_if_present(emu, method_ptr);
    let path = read_ansi_string_if_present(emu, path_ptr);
    let version = read_ansi_string_if_present(emu, version_ptr);
    let referrer = read_ansi_string_if_present(emu, referrer_ptr);
    let access = read_ansi_string_if_present(emu, access_ptr);

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

    pop_stack32(emu, 8);

    let uri = format!("HttpOpenRequestA://{}", path);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub(super) fn http_open_request_w(emu: &mut emu::Emu) {
    let conn_hndl = read_stack_dword(emu, 0, "wininet!HttpOpenRequestW cannot read hndl") as u64;
    let method_ptr =
        read_stack_dword(emu, 4, "wininet!HttpOpenRequestW cannot read method_ptr") as u64;
    let path_ptr = read_stack_dword(emu, 8, "wininet!HttpOpenRequestW cannot read path_ptr") as u64;
    let version_ptr =
        read_stack_dword(emu, 12, "wininet!HttpOpenRequestW cannot read version_ptr") as u64;
    let referrer_ptr =
        read_stack_dword(emu, 16, "wininet!HttpOpenRequestW cannot read referrer_ptr") as u64;
    let access_ptr =
        read_stack_dword(emu, 20, "wininet!HttpOpenRequestW cannot read access_ptr") as u64;
    let flags = read_stack_dword(emu, 24, "wininet!HttpOpenRequestW cannot read flags") as u64;
    let _ctx = read_stack_dword(emu, 28, "wininet!HttpOpenRequestW cannot read ctx");

    let method = read_wide_string_if_present(emu, method_ptr);
    let path = read_wide_string_if_present(emu, path_ptr);
    let version = read_wide_string_if_present(emu, version_ptr);
    let referrer = read_wide_string_if_present(emu, referrer_ptr);
    let access = read_wide_string_if_present(emu, access_ptr);

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

    if flags & constants::INTERNET_FLAG_SECURE != 0 {
        log::trace!("\tssl communication.");
    }

    pop_stack32(emu, 8);

    let uri = format!("HttpOpenRequestW://{}", path);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub(super) fn internet_set_option_a(emu: &mut emu::Emu) {
    let inet_hndl =
        read_stack_dword(emu, 0, "wininet!InternetSetOptionA cannot read inet_hndl") as u64;
    let option = read_stack_dword(emu, 4, "wininet!InternetSetOptionA cannot read option");
    let buffer = read_stack_dword(emu, 8, "wininet!InternetSetOptionA cannot read buffer") as u64;
    let len = read_stack_dword(emu, 12, "wininet!InternetSetOptionA cannot read len");

    let mut buffer_content = String::new();
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

    pop_stack32(emu, 4);

    emu.regs_mut().rax = 1;
}

pub(super) fn internet_set_option_w(emu: &mut emu::Emu) {
    let inet_hndl =
        read_stack_dword(emu, 0, "wininet!InternetSetOptionW cannot read inet_hndl") as u64;
    let option = read_stack_dword(emu, 4, "wininet!InternetSetOptionW cannot read option");
    let buffer = read_stack_dword(emu, 8, "wininet!InternetSetOptionW cannot read buffer") as u64;
    let len = read_stack_dword(emu, 12, "wininet!InternetSetOptionW cannot read len");

    let mut buffer_content = String::new();
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

    pop_stack32(emu, 4);

    emu.regs_mut().rax = 1;
}

pub(super) fn http_send_request_a(emu: &mut emu::Emu) {
    let req_hndl = read_stack_dword(emu, 0, "wininet!HttpSendRequestA cannot read req_hndl") as u64;
    let hdrs_ptr = read_stack_dword(emu, 4, "wininet!HttpSendRequestA cannot read hdrs_ptr") as u64;
    let _hdrs_len = read_stack_dword(emu, 8, "wininet!HttpSendRequestA cannot read hdrs_len");
    let opt_ptr = read_stack_dword(emu, 12, "wininet!HttpSendRequestA cannot read opt_ptr") as u64;
    let _opt_len = read_stack_dword(emu, 16, "wininet!HttpSendRequestA cannot read opt_len");

    let hdrs = emu.maps.read_string(hdrs_ptr);
    let opt = emu.maps.read_string(opt_ptr);

    log_red!(emu, "wininet!HttpSendRequestA hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    pop_stack32(emu, 5);

    emu.regs_mut().rax = 1;
}

pub(super) fn http_send_request_w(emu: &mut emu::Emu) {
    let req_hndl = read_stack_dword(emu, 0, "wininet!HttpSendRequestW cannot read req_hndl") as u64;
    let hdrs_ptr = read_stack_dword(emu, 4, "wininet!HttpSendRequestW cannot read hdrs_ptr") as u64;
    let _hdrs_len = read_stack_dword(emu, 8, "wininet!HttpSendRequestW cannot read hdrs_len");
    let opt_ptr = read_stack_dword(emu, 12, "wininet!HttpSendRequestW cannot read opt_ptr") as u64;
    let _opt_len = read_stack_dword(emu, 16, "wininet!HttpSendRequestW cannot read opt_len");

    let hdrs = emu.maps.read_wide_string(hdrs_ptr);
    let opt = emu.maps.read_wide_string(opt_ptr);

    log_red!(emu, "wininet!HttpSendRequestW hdrs: {} opt: {}", hdrs, opt);

    if !helper::handler_exist(req_hndl) {
        log::trace!("\tinvalid handle.");
    }

    pop_stack32(emu, 5);

    emu.regs_mut().rax = 1;
}

pub(super) fn http_query_info_a(emu: &mut emu::Emu) {
    let handle = read_stack_dword(emu, 0, "wininet!HttpQueryInfoA cannot read hrequest") as u64;
    let info_level = read_stack_dword(emu, 4, "wininet!HttpQueryInfoA cannot read infolvl");
    let buffer = read_stack_dword(emu, 8, "wininet!HttpQueryInfoA cannot read buffer") as u64;
    let buffer_len_ptr =
        read_stack_dword(emu, 12, "wininet!HttpQueryInfoA cannot read buffer len") as u64;
    let _index = read_stack_dword(emu, 16, "wininet!HttpQueryInfoA cannot read index");
    let level_raw = info_level & 0x0000FFFF;

    log_red!(
        emu,
        "wininet!HttpQueryInfoA handle: 0x{:x} level: 0x{:x} (raw: {})",
        handle,
        info_level,
        level_raw
    );

    pop_stack32(emu, 5);

    let response: Option<&str> = match level_raw {
        1 => Some("text/html"),
        5 => Some("1024"),
        19 => Some("200"),
        21 => Some("HTTP/1.1 200 OK"),
        22 => Some("HTTP/1.1 200 OK\r\n"),
        _ => None,
    };

    match response {
        Some(val) => {
            if buffer != 0 {
                emu.maps.write_string(buffer, val);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, val.len() as u32);
            }
            emu.regs_mut().rax = 1;
        }
        None => {
            emu.regs_mut().rax = 0;
        }
    }
}

pub(super) fn http_query_info_w(emu: &mut emu::Emu) {
    let handle = read_stack_dword(emu, 0, "wininet!HttpQueryInfoW cannot read hrequest") as u64;
    let info_level = read_stack_dword(emu, 4, "wininet!HttpQueryInfoW cannot read infolvl");
    let buffer = read_stack_dword(emu, 8, "wininet!HttpQueryInfoW cannot read buffer") as u64;
    let buffer_len_ptr =
        read_stack_dword(emu, 12, "wininet!HttpQueryInfoW cannot read buffer len") as u64;
    let _index = read_stack_dword(emu, 16, "wininet!HttpQueryInfoW cannot read index");
    let level_raw = info_level & 0x0000FFFF;

    log_red!(
        emu,
        "wininet!HttpQueryInfoW handle: 0x{:x} level: 0x{:x} (raw: {})",
        handle,
        info_level,
        level_raw
    );

    pop_stack32(emu, 5);

    let response: Option<&str> = match level_raw {
        1 => Some("text/html"),
        5 => Some("1024"),
        19 => Some("200"),
        21 => Some("HTTP/1.1 200 OK"),
        22 => Some("HTTP/1.1 200 OK\r\n"),
        _ => None,
    };

    match response {
        Some(val) => {
            if buffer != 0 {
                emu.maps.write_wide_string(buffer, val);
            }
            if buffer_len_ptr != 0 {
                emu.maps.write_dword(buffer_len_ptr, (val.len() * 2) as u32);
            }
            emu.regs_mut().rax = 1;
        }
        None => {
            emu.regs_mut().rax = 0;
        }
    }
}
