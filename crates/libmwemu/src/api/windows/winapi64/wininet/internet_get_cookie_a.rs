use crate::emu;

pub fn InternetGetCookieA(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let data_ptr = emu.regs().r8;
    let size_ptr = emu.regs().r9;
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!InternetGetCookieA url:'{}'", url);
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    emu.set_last_error(259);
    emu.regs_mut().rax = 0;
}
