use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateJobSet(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateJobSet");
    emu.regs_mut().rax = helper::handler_create("CreateJobSet");
    stdcall_ret(emu, 3);
}
