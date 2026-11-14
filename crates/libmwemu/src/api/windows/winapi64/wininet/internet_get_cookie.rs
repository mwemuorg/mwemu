use crate::emu;
use crate::winapi::common::strings;

pub fn InternetGetCookieA(emu: &mut emu::Emu) {
    internet_get_cookie(emu, "InternetGetCookieA", false);
}

fn internet_get_cookie(emu: &mut emu::Emu, api: &str, wide: bool) {
    let url_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let data_ptr = emu.regs().r8;
    let size_ptr = emu.regs().r9;
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}'", url);
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    emu.set_last_error(259);
    emu.regs_mut().rax = 0;
}

pub fn InternetGetCookieW(emu: &mut emu::Emu) {
    internet_get_cookie(emu, "InternetGetCookieW", true);
}
