use crate::emu;
use crate::winapi::common::strings;

pub fn HttpAddRequestHeadersA(emu: &mut emu::Emu) {
    http_add_request_headers(emu, "HttpAddRequestHeadersA", false);
}

fn http_add_request_headers(emu: &mut emu::Emu, api: &str, wide: bool) {
    let handle = emu.regs().rcx;
    let headers_ptr = emu.regs().rdx;
    let headers_len = emu.regs().r8 as i32;
    let modifiers = emu.regs().r9 as u32;
    let hdrs = if headers_ptr != 0 {
        strings::read_text(emu, headers_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} hdrs:'{}' mod:0x{:x}",
        handle,
        hdrs,
        modifiers
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn HttpAddRequestHeadersW(emu: &mut emu::Emu) {
    http_add_request_headers(emu, "HttpAddRequestHeadersW", true);
}
