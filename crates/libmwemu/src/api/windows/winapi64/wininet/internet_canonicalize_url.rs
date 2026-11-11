use crate::emu;

pub fn InternetCanonicalizeUrlA(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let out_ptr = emu.regs().rdx;
    let out_len_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let url = if url_ptr != 0 {
        emu.maps.read_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetCanonicalizeUrlA url:'{}' flags:0x{:x}",
        url,
        flags
    );
    if out_ptr != 0 {
        emu.maps.write_string(out_ptr, &url);
    }
    if out_len_ptr != 0 {
        emu.maps.write_dword(out_len_ptr, url.len() as u32);
    }
    emu.regs_mut().rax = 1;
}

pub fn InternetCanonicalizeUrlW(emu: &mut emu::Emu) {
    let url_ptr = emu.regs().rcx;
    let out_ptr = emu.regs().rdx;
    let out_len_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let url = if url_ptr != 0 {
        emu.maps.read_wide_string(url_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!InternetCanonicalizeUrlW url:'{}' flags:0x{:x}",
        url,
        flags
    );
    if out_ptr != 0 {
        emu.maps.write_wide_string(out_ptr, &url);
    }
    if out_len_ptr != 0 {
        emu.maps.write_dword(out_len_ptr, url.len() as u32);
    }
    emu.regs_mut().rax = 1;
}
