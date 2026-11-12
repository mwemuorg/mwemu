use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn InternetConnectA(emu: &mut emu::Emu) {
    internet_connect(emu, "InternetConnectA", false);
}

pub fn InternetConnectW(emu: &mut emu::Emu) {
    internet_connect(emu, "InternetConnectW", true);
}

fn internet_connect(emu: &mut emu::Emu, api: &str, wide: bool) {
    let internet_hndl = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8;
    let login_ptr = emu.regs().r9;
    let passw_ptr = WinApiAbi::arg(emu, 4);
    let service = WinApiAbi::arg(emu, 5);
    let flags = WinApiAbi::arg(emu, 6);
    let ctx = WinApiAbi::arg(emu, 7);

    let mut server = "".to_string();
    let mut login = "".to_string();
    let mut passw = "".to_string();

    if server_ptr != 0 {
        server = strings::read_text(emu, server_ptr, wide);
    }
    if login_ptr != 0 {
        login = strings::read_text(emu, login_ptr, wide);
    }
    if passw_ptr != 0 {
        passw = strings::read_text(emu, passw_ptr, wide);
    }

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

    /*
    if emu.cfg.endpoint {
        endpoint::http_set_serverport(&server, port as u16);
    }*/

    let uri = format!("{api}://{}:{}", server, port);
    emu.regs_mut().rax = helper::handler_create(&uri); // connect handle
}
