use crate::emu;

pub fn GetComPlusPackageInstallStatus(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetComPlusPackageInstallStatus => 0");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
