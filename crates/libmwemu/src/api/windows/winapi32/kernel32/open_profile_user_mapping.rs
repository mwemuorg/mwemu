use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn OpenProfileUserMapping(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenProfileUserMapping");
    emu.regs_mut().rax = helper::handler_create("OpenProfileUserMapping");
    stdcall_ret(emu, 1);
}
