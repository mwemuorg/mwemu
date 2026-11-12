use crate::emu;

pub fn InternetGetPerSiteCookieDecisionA(emu: &mut emu::Emu) {
    internet_get_per_site_cookie_decision(emu, "InternetGetPerSiteCookieDecisionA");
}

fn internet_get_per_site_cookie_decision(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn InternetGetPerSiteCookieDecisionW(emu: &mut emu::Emu) {
    internet_get_per_site_cookie_decision(emu, "InternetGetPerSiteCookieDecisionW");
}
