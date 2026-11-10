use crate::emu;
use crate::windows::constants::errors::*;

pub fn GetComPlusPackageInstallStatus(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetComPlusPackageInstallStatus");

    emu.regs_mut().rax = ERROR_SUCCESS;
}
