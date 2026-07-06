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
    let _hrequest = read_stack_dword(emu, 0, "wininet!HttpQueryInfoA cannot read hrequest") as u64;
    let _infolvl = read_stack_dword(emu, 4, "wininet!HttpQueryInfoA cannot read infolvl") as u64;
    let buff = read_stack_dword(emu, 8, "wininet!HttpQueryInfoA cannot read buffer") as u64;
    let buff_len =
        read_stack_dword(emu, 12, "wininet!HttpQueryInfoA cannot read buffer len") as u64;
    let _index = read_stack_dword(emu, 16, "wininet!HttpQueryInfoA cannot read index") as u64;

    log_red!(
        emu,
        "wininet!HttpQueryInfoA buff: 0x{:x} sz:{}",
        buff,
        buff_len
    );

    pop_stack32(emu, 5);

    emu.regs_mut().rax = 1;
}
