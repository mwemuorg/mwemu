use crate::emu;

pub fn InternetSetCookieW(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let data_ptr = emu.regs().r8;
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
    let data = if data_ptr != 0 {
        emu.maps.read_wide_string(data_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetSetCookieW url:'{}' name:'{}' data:'{}'",
        url,
        name,
        data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
