
use crate::emu;
use crate::winapi::helper;
pub fn InstallELAMCertificateInfo(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!InstallELAMCertificateInfo hFile:0x{:x}",
        h_file
    );
    let handle = helper::handler_create("InstallElamCertificateInfo");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
