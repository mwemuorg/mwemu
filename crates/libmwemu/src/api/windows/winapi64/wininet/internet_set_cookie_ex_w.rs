use crate::emu;

pub fn InternetSetCookieExW(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let data_ptr = emu.regs().r8;
    let flags = emu.regs().r9;
    let url = if url_ptr != 0 {
        emu.maps.read_wide_string(url_ptr)
    } else {
        String::new()
    };
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetSetCookieExW url:'{}' name:'{}' flags:0x{:x}",
        url,
        name,
        flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
