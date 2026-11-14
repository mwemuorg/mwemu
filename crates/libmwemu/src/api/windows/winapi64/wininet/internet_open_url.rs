use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn InternetOpenUrlA(emu: &mut emu::Emu) {
    internet_open_url(emu, "InternetOpenUrlA", false);
}

pub fn InternetOpenUrlW(emu: &mut emu::Emu) {
    internet_open_url(emu, "InternetOpenUrlW", true);
}

fn internet_open_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let inet_handle = emu.regs().rcx;
    let url_ptr = emu.regs().rdx;
    let headers_ptr = emu.regs().r8;
    let headers_len = emu.regs().r9 as u32;
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} inet:0x{:x} url:'{}'", inet_handle, url);
    let uri = format!("{api}://{}", url);
    let handle = helper::handler_create(&uri);
    emu.regs_mut().rax = handle;
}
