use crate::emu;
use crate::winapi::common::strings;

pub fn FtpRenameFileA(emu: &mut emu::Emu) {
    ftp_rename_file(emu, "FtpRenameFileA", false);
}

fn ftp_rename_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let old_ptr = emu.regs().rdx;
    let new_ptr = emu.regs().r8;
    let old_name = if old_ptr != 0 {
        strings::read_text(emu, old_ptr, wide)
    } else {
        String::new()
    };
    let new_name = if new_ptr != 0 {
        strings::read_text(emu, new_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} old:'{}' new:'{}'",
        ftp_handle,
        old_name,
        new_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FtpRenameFileW(emu: &mut emu::Emu) {
    ftp_rename_file(emu, "FtpRenameFileW", true);
}
