use crate::emu;

pub fn FtpFindFirstFileA(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let search_ptr = emu.regs().rdx;
    let find_data_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let search = if search_ptr != 0 {
        emu.maps.read_string(search_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpFindFirstFileA handle:0x{:x} search:'{}'",
        ftp_handle,
        search
    );
    emu.set_last_error(18);
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
}

pub fn FtpFindFirstFileW(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let search_ptr = emu.regs().rdx;
    let find_data_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let search = if search_ptr != 0 {
        emu.maps.read_wide_string(search_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpFindFirstFileW handle:0x{:x} search:'{}'",
        ftp_handle,
        search
    );
    emu.set_last_error(18);
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
}
