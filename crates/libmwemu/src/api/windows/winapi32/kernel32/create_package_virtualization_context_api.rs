use crate::emu;
use crate::winapi::helper;

pub fn CreatePackageVirtualizationContext_api(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreatePackageVirtualizationContext");
    emu.regs_mut().rax = helper::handler_create("CreatePackageVirtualizationContext");
}
