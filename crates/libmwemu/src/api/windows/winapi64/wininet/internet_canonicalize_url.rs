use crate::emu;
use crate::winapi::common::strings;

pub fn InternetCanonicalizeUrlA(emu: &mut emu::Emu) {
    internet_canonicalize_url(emu, "InternetCanonicalizeUrlA", false);
}

fn internet_canonicalize_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let url_ptr = emu.regs().rcx;
    let out_ptr = emu.regs().rdx;
    let out_len_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let url = if url_ptr != 0 {
        strings::read_text(emu, url_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} url:'{}' flags:0x{:x}", url, flags);
    if out_ptr != 0 {
        strings::write_string(emu, out_ptr, &url, wide);
    }
    if out_len_ptr != 0 {
        emu.maps.write_dword(out_len_ptr, url.len() as u32);
    }
    emu.regs_mut().rax = 1;
}

pub fn InternetCanonicalizeUrlW(emu: &mut emu::Emu) {
    internet_canonicalize_url(emu, "InternetCanonicalizeUrlW", true);
}
