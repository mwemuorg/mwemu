use crate::emu;
use crate::winapi::helper;

pub fn OpenPrivateNamespaceW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenPrivateNamespaceW");
    emu.regs_mut().rax = helper::handler_create("OpenPrivateNamespaceW");
}
