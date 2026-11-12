use crate::emu;
use crate::winapi::common::strings;

pub fn FtpDeleteFileA(emu: &mut emu::Emu) {
    ftp_delete_file(emu, "FtpDeleteFileA", false);
}

fn ftp_delete_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let file_ptr = emu.regs().rdx;
    let file = if file_ptr != 0 {
        strings::read_text(emu, file_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} file:'{}'",
        ftp_handle,
        file
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FtpDeleteFileW(emu: &mut emu::Emu) {
    ftp_delete_file(emu, "FtpDeleteFileW", true);
}
