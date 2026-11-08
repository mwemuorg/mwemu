use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateFile3(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFile3");
    emu.regs_mut().rax = helper::handler_create("CreateFile3");
    stdcall_ret(emu, 1);
}
