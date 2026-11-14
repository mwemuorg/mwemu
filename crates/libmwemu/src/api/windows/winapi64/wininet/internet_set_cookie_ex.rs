use crate::emu;
use crate::winapi::common::strings;

pub fn InternetSetCookieExA(emu: &mut emu::Emu) {
    internet_set_cookie_ex(emu, "InternetSetCookieExA", false);
}

fn internet_set_cookie_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let url_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let data_ptr = emu.regs().r8;
    let flags = emu.regs().r9;
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} url:'{}' name:'{}' flags:0x{:x}",
        url,
        name,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetSetCookieExW(emu: &mut emu::Emu) {
    internet_set_cookie_ex(emu, "InternetSetCookieExW", true);
}
