use crate::emu;
use crate::winapi::common::strings;

pub fn FtpGetCurrentDirectoryA(emu: &mut emu::Emu) {
    ftp_get_current_directory(emu, "FtpGetCurrentDirectoryA", false);
}

fn ftp_get_current_directory(emu: &mut emu::Emu, api: &str, wide: bool) {
    let handle = emu.regs().rcx;
    let dir_ptr = emu.regs().rdx;
    let dir_len_ptr = emu.regs().r8;
    log_red!(emu, "wininet!{api} handle:0x{:x}", handle);
    if dir_ptr != 0 {
        strings::write_string(emu, dir_ptr, "/", wide);
    }
    if dir_len_ptr != 0 {
        emu.maps.write_dword(dir_len_ptr, 1);
    }
    emu.regs_mut().rax = 1;
}

pub fn FtpGetCurrentDirectoryW(emu: &mut emu::Emu) {
    ftp_get_current_directory(emu, "FtpGetCurrentDirectoryW", true);
}
