use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateIoRing(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateIoRing");
    emu.regs_mut().rax = helper::handler_create("CreateIoRing");
    stdcall_ret(emu, 1);
}
