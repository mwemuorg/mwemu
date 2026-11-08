use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateBoundaryDescriptorW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateBoundaryDescriptorW");
    emu.regs_mut().rax = helper::handler_create("CreateBoundaryDescriptorW");
    stdcall_ret(emu, 1);
}
