use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn InstallELAMCertificateInfo(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!InstallELAMCertificateInfo hFile:0x{:x}",
        h_file
    );
    let handle = helper::handler_create("InstallElamCertificateInfo");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
