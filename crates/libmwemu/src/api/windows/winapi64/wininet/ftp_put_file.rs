use crate::emu;
use crate::winapi::common::strings;

pub fn FtpPutFileA(emu: &mut emu::Emu) {
    ftp_put_file(emu, "FtpPutFileA", false);
}

fn ftp_put_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let local_ptr = emu.regs().rdx;
    let remote_ptr = emu.regs().r8;
    let local = if local_ptr != 0 {
        strings::read_text(emu, local_ptr, wide)
    } else {
        String::new()
    };
    let remote = if remote_ptr != 0 {
        strings::read_text(emu, remote_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} local:'{}' remote:'{}'",
        ftp_handle,
        local,
        remote
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FtpPutFileW(emu: &mut emu::Emu) {
    ftp_put_file(emu, "FtpPutFileW", true);
}
