use crate::emu;
use crate::winapi::common::strings;

pub fn InternetCheckConnectionA(emu: &mut emu::Emu) {
    internet_check_connection(emu, "InternetCheckConnectionA", false);
}

fn internet_check_connection(emu: &mut emu::Emu, api: &str, wide: bool) {
    let url_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}' flags:0x{:x}", url, flags);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetCheckConnectionW(emu: &mut emu::Emu) {
    internet_check_connection(emu, "InternetCheckConnectionW", true);
}
