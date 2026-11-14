use crate::emu;

pub fn InternetShowSecurityInfoByURL(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!InternetShowSecurityInfoByURL arg1:0x{:x}",
        arg1
    );
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn InternetShowSecurityInfoByURLA(emu: &mut emu::Emu) {
    internet_show_security_info_by_url(emu, "InternetShowSecurityInfoByURLA");
}

fn internet_show_security_info_by_url(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn InternetShowSecurityInfoByURLW(emu: &mut emu::Emu) {
    internet_show_security_info_by_url(emu, "InternetShowSecurityInfoByURLW");
}
