use crate::emu;

pub fn GetCurrentPackageVirtualizationContext(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetCurrentPackageVirtualizationContext => 0");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
