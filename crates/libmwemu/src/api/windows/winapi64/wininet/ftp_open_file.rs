use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn FtpOpenFileA(emu: &mut emu::Emu) {
    ftp_open_file(emu, "FtpOpenFileA", false);
}

fn ftp_open_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let filename_ptr = emu.regs().rdx;
    let access = emu.regs().r8 as u32;
    let flags = emu.regs().r9 as u32;
    let filename = if filename_ptr != 0 {
        strings::read_text(emu, filename_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} file:'{}' access:0x{:x}",
        ftp_handle,
        filename,
        access
    );
    let uri = format!("ftp://{}", filename);
    emu.regs_mut().rax = helper::handler_create(&uri);
}

pub fn FtpOpenFileW(emu: &mut emu::Emu) {
    ftp_open_file(emu, "FtpOpenFileW", true);
}
