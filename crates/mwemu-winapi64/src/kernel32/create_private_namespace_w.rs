use crate::emu;
use crate::winapi::helper;

pub fn CreatePrivateNamespaceW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreatePrivateNamespaceW");
    emu.regs_mut().rax = helper::handler_create("CreatePrivateNamespaceW");
}
