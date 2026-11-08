use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreatePackageVirtualizationContext(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreatePackageVirtualizationContext");
    emu.regs_mut().rax = helper::handler_create("CreatePackageVirtualizationContext");
    stdcall_ret(emu, 1);
}
