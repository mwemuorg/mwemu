use crate::emu;
use crate::winapi::common::strings;

pub fn InternetSetPerSiteCookieDecisionA(emu: &mut emu::Emu) {
    internet_set_per_site_cookie_decision(emu, "InternetSetPerSiteCookieDecisionA", false);
}

fn internet_set_per_site_cookie_decision(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetSetPerSiteCookieDecisionW(emu: &mut emu::Emu) {
    internet_set_per_site_cookie_decision(emu, "InternetSetPerSiteCookieDecisionW", true);
}
