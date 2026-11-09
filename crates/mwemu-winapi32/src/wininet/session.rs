use crate::emu;
use crate::winapi::helper;

use super::{
    pop_stack32, read_ansi_string_if_present, read_stack_dword, read_wide_string_if_present,
};

pub(super) fn internet_open_a(emu: &mut emu::Emu) {
    let uagent_ptr =
        read_stack_dword(emu, 0, "wininet!InternetOpenA  cannot read uagent_ptr") as u64;
    let _access = read_stack_dword(emu, 4, "wininet!InternetOpenA  cannot read access") as u64;
    let proxy_ptr = read_stack_dword(emu, 8, "wininet!InternetOpenA  cannot read proxy_ptr") as u64;
    let proxybypass_ptr = read_stack_dword(
        emu,
        12,
        "wininet!InternetOpenA  cannot read proxybypass_ptr",
    ) as u64;
    let _flags = read_stack_dword(emu, 16, "wininet!InternetOpenA  cannot read flags");

    let uagent = read_ansi_string_if_present(emu, uagent_ptr);
    let proxy = read_ansi_string_if_present(emu, proxy_ptr);
    let proxy_bypass = read_ansi_string_if_present(emu, proxybypass_ptr);

    log_red!(
        emu,
        "wininet!InternetOpenA uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    pop_stack32(emu, 5);

    let uri = format!("uagent://{}", uagent);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub(super) fn internet_open_w(emu: &mut emu::Emu) {
    let uagent_ptr =
        read_stack_dword(emu, 0, "wininet!InternetOpenW  cannot read uagent_ptr") as u64;
    let _access = read_stack_dword(emu, 4, "wininet!InternetOpenW  cannot read access") as u64;
    let proxy_ptr = read_stack_dword(emu, 8, "wininet!InternetOpenW  cannot read proxy_ptr") as u64;
    let proxybypass_ptr = read_stack_dword(
        emu,
        12,
        "wininet!InternetOpenW  cannot read proxybypass_ptr",
    ) as u64;
    let _flags = read_stack_dword(emu, 16, "wininet!InternetOpenW  cannot read flags");

    let uagent = read_wide_string_if_present(emu, uagent_ptr);
    let proxy = read_wide_string_if_present(emu, proxy_ptr);
    let proxy_bypass = read_wide_string_if_present(emu, proxybypass_ptr);

    log_red!(
        emu,
        "wininet!InternetOpenW uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    pop_stack32(emu, 5);

    emu.regs_mut().rax = helper::handler_create("InternetOpenW");
}

pub(super) fn internet_connect_a(emu: &mut emu::Emu) {
    let internet_hndl =
        read_stack_dword(emu, 0, "wininet!InternetConnectA cannot read hndl") as u64;
    let server_ptr =
        read_stack_dword(emu, 4, "wininet!InternetConnectA cannot read server_ptr") as u64;
    let port = read_stack_dword(emu, 8, "wininet!InternetConnectA cannot read port");
    let login_ptr =
        read_stack_dword(emu, 12, "wininet!InternetConnectA cannot read login_ptr") as u64;
    let passw_ptr =
        read_stack_dword(emu, 16, "wininet!InternetConnectA cannot read passw_ptr") as u64;
    let _service = read_stack_dword(emu, 20, "wininet!InternetConnectA cannot read service");
    let _flags = read_stack_dword(emu, 24, "wininet!InternetConnectA cannot read flags");
    let _ctx = read_stack_dword(emu, 28, "wininet!InternetConnectA cannot read ctx");

    let server = read_ansi_string_if_present(emu, server_ptr);
    let login = read_ansi_string_if_present(emu, login_ptr);
    let passw = read_ansi_string_if_present(emu, passw_ptr);

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

    pop_stack32(emu, 8);

    let uri = format!("InternetConnectA://{}", server);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub(super) fn internet_connect_w(emu: &mut emu::Emu) {
    let internet_hndl =
        read_stack_dword(emu, 0, "wininet!InternetConnectW cannot read hndl") as u64;
    let server_ptr =
        read_stack_dword(emu, 4, "wininet!InternetConnectW cannot read server_ptr") as u64;
    let port = read_stack_dword(emu, 8, "wininet!InternetConnectW cannot read port");
    let login_ptr =
        read_stack_dword(emu, 12, "wininet!InternetConnectW cannot read login_ptr") as u64;
    let passw_ptr =
        read_stack_dword(emu, 16, "wininet!InternetConnectW cannot read passw_ptr") as u64;
    let _service = read_stack_dword(emu, 20, "wininet!InternetConnectW cannot read service");
    let _flags = read_stack_dword(emu, 24, "wininet!InternetConnectW cannot read flags");
    let _ctx = read_stack_dword(emu, 28, "wininet!InternetConnectW cannot read ctx");

    let server = read_wide_string_if_present(emu, server_ptr);
    let login = read_wide_string_if_present(emu, login_ptr);
    let passw = read_wide_string_if_present(emu, passw_ptr);

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

    pop_stack32(emu, 8);

    let uri = format!("InternetConnectW://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
