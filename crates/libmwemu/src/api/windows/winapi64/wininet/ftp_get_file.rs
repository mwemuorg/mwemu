use crate::emu;
use crate::winapi::common::strings;

pub fn FtpGetFileA(emu: &mut emu::Emu) {
    ftp_get_file(emu, "FtpGetFileA", false);
}

fn ftp_get_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let remote_ptr = emu.regs().rdx;
    let local_ptr = emu.regs().r8;
    let remote = if remote_ptr != 0 {
        strings::read_text(emu, remote_ptr, wide)
    } else {
        String::new()
    };
    let local = if local_ptr != 0 {
        strings::read_text(emu, local_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} remote:'{}' local:'{}'",
        ftp_handle,
        remote,
        local
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FtpGetFileW(emu: &mut emu::Emu) {
    ftp_get_file(emu, "FtpGetFileW", true);
}
