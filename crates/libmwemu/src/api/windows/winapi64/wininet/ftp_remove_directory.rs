use crate::emu;

pub fn FtpRemoveDirectoryA(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let dir_ptr = emu.regs().rdx;
    let dir = if dir_ptr != 0 {
        emu.maps.read_string(dir_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpRemoveDirectoryA handle:0x{:x} dir:'{}'",
        ftp_handle,
        dir
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FtpRemoveDirectoryW(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let dir_ptr = emu.regs().rdx;
    let dir = if dir_ptr != 0 {
        emu.maps.read_wide_string(dir_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpRemoveDirectoryW handle:0x{:x} dir:'{}'",
        ftp_handle,
        dir
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
