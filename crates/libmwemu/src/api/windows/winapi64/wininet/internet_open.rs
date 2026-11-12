use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn InternetOpenA(emu: &mut emu::Emu) {
    internet_open(emu, "InternetOpenA", false);
}

pub fn InternetOpenW(emu: &mut emu::Emu) {
    internet_open(emu, "InternetOpenW", true);
}

fn internet_open(emu: &mut emu::Emu, api: &str, wide: bool) {
    let uagent_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let proxy_ptr = emu.regs().r8;
    let proxybypass_ptr = emu.regs().r9;
    let flags = WinApiAbi::arg(emu, 4);
    let mut uagent = "".to_string();
    let mut proxy = "".to_string();
    let mut proxy_bypass = "".to_string();
    if uagent_ptr != 0 {
        uagent = strings::read_text(emu, uagent_ptr, wide);
    }
    if proxy_ptr != 0 {
        proxy = strings::read_text(emu, proxy_ptr, wide);
    }
    if proxybypass_ptr != 0 {
        proxy_bypass = strings::read_text(emu, proxybypass_ptr, wide);
    }
    log_red!(
        emu,
        "wininet!{api} uagent: {} proxy: {} {}",
        uagent,
        proxy,
        proxy_bypass
    );
    if wide {
        emu.regs_mut().rax = helper::handler_create(&format!("{api}://"));
    } else {
        let uri = format!("{api}://{}", uagent);
        emu.regs_mut().rax = helper::handler_create(&uri);
    }
}
