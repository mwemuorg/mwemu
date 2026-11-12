use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn FtpOpenFileA(emu: &mut emu::Emu) {
    ftp_open_file(emu, "FtpOpenFileA", false);
}

pub fn FtpOpenFileW(emu: &mut emu::Emu) {
    ftp_open_file(emu, "FtpOpenFileW", true);
}

fn ftp_open_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    if wide {
        emu.regs_mut().rax = helper::handler_create("ftpopenfilew");
    } else {
        emu.regs_mut().rax = helper::handler_create("ftpopenfilea");
    }

    stdcall_ret(emu, 5);
}
