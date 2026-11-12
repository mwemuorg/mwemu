use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn InternetEnumPerSiteCookieDecisionA(emu: &mut emu::Emu) {
    internet_enum_per_site_cookie_decision(emu, "InternetEnumPerSiteCookieDecisionA");
}

fn internet_enum_per_site_cookie_decision(emu: &mut emu::Emu, api: &str) {
    let buf_ptr = emu.regs().rcx;
    let buf_size = emu.regs().rdx;
    let index_ptr = emu.regs().r8;
    log_red!(
        emu,
        "wininet!{api} buf:0x{:x} idx:0x{:x}",
        buf_ptr,
        index_ptr
    );
    let out_ptr = WinApiAbi::arg(emu, 2);
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(259);
    emu.regs_mut().rax = 0;
}

pub fn InternetEnumPerSiteCookieDecisionW(emu: &mut emu::Emu) {
    internet_enum_per_site_cookie_decision(emu, "InternetEnumPerSiteCookieDecisionW");
}
