use crate::emu;
use crate::winapi::common::strings;

pub fn FtpFindFirstFileA(emu: &mut emu::Emu) {
    ftp_find_first_file(emu, "FtpFindFirstFileA", false);
}

fn ftp_find_first_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let search_ptr = emu.regs().rdx;
    let find_data_ptr = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let search = if search_ptr != 0 {
        strings::read_text(emu, search_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} search:'{}'",
        ftp_handle,
        search
    );
    emu.set_last_error(18);
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
}

pub fn FtpFindFirstFileW(emu: &mut emu::Emu) {
    ftp_find_first_file(emu, "FtpFindFirstFileW", true);
}
