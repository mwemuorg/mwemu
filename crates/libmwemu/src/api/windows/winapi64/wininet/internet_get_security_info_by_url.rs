use crate::emu;
use crate::winapi::common::strings;

pub fn InternetGetSecurityInfoByURL(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!InternetGetSecurityInfoByURL arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetGetSecurityInfoByURLA(emu: &mut emu::Emu) {
    internet_get_security_info_by_url(emu, "InternetGetSecurityInfoByURLA", false);
}

fn internet_get_security_info_by_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "wininet!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetGetSecurityInfoByURLW(emu: &mut emu::Emu) {
    internet_get_security_info_by_url(emu, "InternetGetSecurityInfoByURLW", true);
}
