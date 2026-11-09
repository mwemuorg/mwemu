use crate::emu;
use crate::winapi::helper;

pub fn CreateBoundaryDescriptorW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateBoundaryDescriptorW");
    emu.regs_mut().rax = helper::handler_create("CreateBoundaryDescriptorW");
}
