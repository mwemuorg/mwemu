use crate::emu;
use crate::winapi::helper;

pub fn InternetOpenUrlA(emu: &mut emu::Emu) {
    let inet_handle = emu.regs().rcx;
    let url_ptr = emu.regs().rdx;
    let headers_ptr = emu.regs().r8;
    let headers_len = emu.regs().r9 as u32;
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetOpenUrlA inet:0x{:x} url:'{}'",
        inet_handle,
        url
    );
    let uri = format!("InternetOpenUrlA://{}", url);
    let handle = helper::handler_create(&uri);
    emu.regs_mut().rax = handle;
}

pub fn InternetOpenUrlW(emu: &mut emu::Emu) {
    let inet_handle = emu.regs().rcx;
    let url_ptr = emu.regs().rdx;
    let headers_ptr = emu.regs().r8;
    let headers_len = emu.regs().r9 as u32;
    let url = if url_ptr != 0 {
        emu.maps.read_wide_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetOpenUrlW inet:0x{:x} url:'{}'",
        inet_handle,
        url
    );
    let uri = format!("InternetOpenUrlW://{}", url);
    let handle = helper::handler_create(&uri);
    emu.regs_mut().rax = handle;
}
