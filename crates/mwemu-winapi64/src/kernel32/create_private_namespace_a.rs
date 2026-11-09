use crate::emu;
use crate::winapi::helper;

pub fn CreatePrivateNamespaceA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreatePrivateNamespaceA");
    emu.regs_mut().rax = helper::handler_create("CreatePrivateNamespaceA");
}
