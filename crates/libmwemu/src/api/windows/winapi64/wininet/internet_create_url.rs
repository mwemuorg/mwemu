use crate::emu;
use crate::winapi::common::strings;

pub fn InternetCreateUrlA(emu: &mut emu::Emu) {
    internet_create_url(emu, "InternetCreateUrlA", false);
}

fn internet_create_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let url_ptr = emu.regs().rcx;
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}'", url);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetCreateUrlW(emu: &mut emu::Emu) {
    internet_create_url(emu, "InternetCreateUrlW", true);
}
