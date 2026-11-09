use crate::emu;
use crate::winapi::helper;

pub fn InternetConnectW(emu: &mut emu::Emu) {
    let internet_hndl = emu.regs().rcx;
    let server_ptr = emu.regs().rdx;
    let port = emu.regs().r8;
    let login_ptr = emu.regs().r9;
    let passw_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let service = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let ctx = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

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
