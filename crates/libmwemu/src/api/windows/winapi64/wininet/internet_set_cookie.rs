use crate::emu;
use crate::winapi::common::strings;

pub fn InternetSetCookieA(emu: &mut emu::Emu) {
    internet_set_cookie(emu, "InternetSetCookieA", false);
}

fn internet_set_cookie(emu: &mut emu::Emu, api: &str, wide: bool) {
    let url_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let data_ptr = emu.regs().r8;
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
    let data = if data_ptr != 0 {
        strings::read_text(emu, data_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} url:'{}' name:'{}' data:'{}'",
        url,
        name,
        data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetSetCookieW(emu: &mut emu::Emu) {
    internet_set_cookie(emu, "InternetSetCookieW", true);
}
