use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreatePrivateNamespaceA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreatePrivateNamespaceA");
    emu.regs_mut().rax = helper::handler_create("CreatePrivateNamespaceA");
    stdcall_ret(emu, 1);
}
