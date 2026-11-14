use crate::emu;
use crate::winapi::common::strings;

pub fn FtpCreateDirectoryA(emu: &mut emu::Emu) {
    ftp_create_directory(emu, "FtpCreateDirectoryA", false);
}

fn ftp_create_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let dir_ptr = emu.regs().rdx;
    let dir = if dir_ptr != 0 {
        strings::read_text(emu, dir_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} handle:0x{:x} dir:'{}'", ftp_handle, dir);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FtpCreateDirectoryW(emu: &mut emu::Emu) {
    ftp_create_directory(emu, "FtpCreateDirectoryW", true);
}
