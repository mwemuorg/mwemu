use crate::emu;

pub fn FtpDeleteFileW(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let file_ptr = emu.regs().rdx;
    let file = if file_ptr != 0 {
        emu.maps.read_wide_string(file_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpDeleteFileW handle:0x{:x} file:'{}'",
        ftp_handle,
        file
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
