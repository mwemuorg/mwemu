use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn internet_open_a(emu: &mut emu::Emu) {
    internet_open(emu, "InternetOpenA", false);
}

pub(super) fn internet_open_w(emu: &mut emu::Emu) {
    internet_open(emu, "InternetOpenW", true);
}

fn internet_open(emu: &mut emu::Emu, api: &str, wide: bool) {
    let uagent_ptr = WinApiAbi::arg(emu, 0);
    let _access = WinApiAbi::arg(emu, 1);
    let proxy_ptr = WinApiAbi::arg(emu, 2);
    let proxybypass_ptr = WinApiAbi::arg(emu, 3);
    let _flags = WinApiAbi::arg(emu, 4) as u32;

    let uagent = strings::read_text(emu, uagent_ptr, wide);
    let proxy = strings::read_text(emu, proxy_ptr, wide);
    let proxy_bypass = strings::read_text(emu, proxybypass_ptr, wide);

    log_red!(
        emu,
        "wininet!{api} uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );

    WinApiAbi::stdcall_ret(emu, 5);

    let uri = if wide {
        api.to_owned()
    } else {
        format!("uagent://{uagent}")
    };
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub(super) fn internet_connect_a(emu: &mut emu::Emu) {
    internet_connect(emu, "InternetConnectA", false);
}

pub(super) fn internet_connect_w(emu: &mut emu::Emu) {
    internet_connect(emu, "InternetConnectW", true);
}

fn internet_connect(emu: &mut emu::Emu, api: &str, wide: bool) {
    let internet_hndl = WinApiAbi::arg(emu, 0);
    let server_ptr = WinApiAbi::arg(emu, 1);
    let port = WinApiAbi::arg(emu, 2) as u32;
    let login_ptr = WinApiAbi::arg(emu, 3);
    let passw_ptr = WinApiAbi::arg(emu, 4);
    let _service = WinApiAbi::arg(emu, 5) as u32;
    let _flags = WinApiAbi::arg(emu, 6) as u32;
    let _ctx = WinApiAbi::arg(emu, 7) as u32;

    let server = strings::read_text(emu, server_ptr, wide);
    let login = strings::read_text(emu, login_ptr, wide);
    let passw = strings::read_text(emu, passw_ptr, wide);

    log_red!(
        emu,
        "wininet!{api} host: {} port: {} login: {} passw: {}",
        server,
        port,
        login,
        passw
    );

    if !helper::handler_exist(internet_hndl) {
        log::trace!("\tinvalid handle.");
    }

    WinApiAbi::stdcall_ret(emu, 8);

    let uri = if wide {
        format!("{api}://{server}:{port}")
    } else {
        format!("{api}://{server}")
    };
    emu.regs_mut().rax = helper::handler_create(&uri);
}
