use crate::emu;
use crate::winapi::helper;

pub fn FtpOpenFileA(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let filename_ptr = emu.regs().rdx;
    let access = emu.regs().r8 as u32;
    let flags = emu.regs().r9 as u32;
    let filename = if filename_ptr != 0 {
        emu.maps.read_string(filename_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpOpenFileA handle:0x{:x} file:'{}' access:0x{:x}",
        ftp_handle,
        filename,
        access
    );
    let uri = format!("ftp://{}", filename);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
