use crate::emu;
use crate::winapi::common::strings;

pub fn FtpCommandA(emu: &mut emu::Emu) {
    ftp_command(emu, "FtpCommandA", false);
}

fn ftp_command(emu: &mut emu::Emu, api: &str, wide: bool) {
    let ftp_handle = emu.regs().rcx;
    let expect_response = emu.regs().rdx as u32;
    let flags = emu.regs().r8 as u32;
    let cmd_ptr = emu.regs().r9;
    let cmd = if cmd_ptr != 0 {
        strings::read_text(emu, cmd_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "wininet!{api} handle:0x{:x} cmd:'{}'", ftp_handle, cmd);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FtpCommandW(emu: &mut emu::Emu) {
    ftp_command(emu, "FtpCommandW", true);
}
