use crate::emu;

pub fn FtpGetFileA(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let remote_ptr = emu.regs().rdx;
    let local_ptr = emu.regs().r8;
    let remote = if remote_ptr != 0 {
        emu.maps.read_string(remote_ptr)
    } else {
        String::new()
    };
    let local = if local_ptr != 0 {
        emu.maps.read_string(local_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpGetFileA handle:0x{:x} remote:'{}' local:'{}'",
        ftp_handle,
        remote,
        local
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
