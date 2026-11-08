use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateActCtxW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateActCtxW");
    emu.regs_mut().rax = helper::handler_create("CreateActCtxW");
    stdcall_ret(emu, 1);
}
