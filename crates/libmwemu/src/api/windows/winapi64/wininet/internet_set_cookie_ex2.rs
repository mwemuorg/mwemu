use crate::emu;

pub fn InternetSetCookieEx2(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let cookie_ptr = emu.regs().rdx;
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    let cookie = if cookie_ptr != 0 {
        emu.maps.read_string(cookie_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetSetCookieEx2 url:'{}' cookie:'{}'",
        url,
        cookie
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // COOKIE_STATE_ACCEPT
}
