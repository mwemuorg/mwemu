use crate::emu;

pub fn InternetCreateUrlA(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!InternetCreateUrlA url:'{}'", url);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetCreateUrlW(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!InternetCreateUrlW url:'{}'", url);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
