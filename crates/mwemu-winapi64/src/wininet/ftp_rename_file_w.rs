use crate::emu;

pub fn FtpRenameFileW(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let old_ptr = emu.regs().rdx;
    let new_ptr = emu.regs().r8;
    let old_name = if old_ptr != 0 {
        emu.maps.read_wide_string(old_ptr)
    } else {
        String::new()
    };
    let new_name = if new_ptr != 0 {
        emu.maps.read_wide_string(new_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpRenameFileW handle:0x{:x} old:'{}' new:'{}'",
        ftp_handle,
        old_name,
        new_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
