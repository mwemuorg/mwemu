use crate::emu;
use crate::winapi::helper;

pub fn InternetOpenA(emu: &mut emu::Emu) {
    let uagent_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let proxy_ptr = emu.regs().r8;
    let proxybypass_ptr = emu.regs().r9;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

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
