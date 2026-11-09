use crate::emu;

pub fn FtpCommandA(emu: &mut emu::Emu) {
    let ftp_handle = emu.regs().rcx;
    let expect_response = emu.regs().rdx as u32;
    let flags = emu.regs().r8 as u32;
    let cmd_ptr = emu.regs().r9;
    let cmd = if cmd_ptr != 0 {
        emu.maps.read_string(cmd_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "wininet!FtpCommandA handle:0x{:x} cmd:'{}'",
        ftp_handle,
        cmd
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
