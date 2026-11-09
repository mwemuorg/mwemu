use crate::emu;

pub fn FtpPutFileW(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let local_ptr = emu.regs().rdx;
    let remote_ptr = emu.regs().r8;
    let local = if local_ptr != 0 {
        emu.maps.read_wide_string(local_ptr)
    } else {
        String::new()
    };
    let remote = if remote_ptr != 0 {
        emu.maps.read_wide_string(remote_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpPutFileW handle:0x{:x} local:'{}' remote:'{}'",
        ftp_handle,
        local,
        remote
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
