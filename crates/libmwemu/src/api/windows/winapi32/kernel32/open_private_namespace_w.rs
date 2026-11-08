use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn OpenPrivateNamespaceW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenPrivateNamespaceW");
    emu.regs_mut().rax = helper::handler_create("OpenPrivateNamespaceW");
    stdcall_ret(emu, 1);
}
