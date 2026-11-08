use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateDirectory2W(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateDirectory2W");
    emu.regs_mut().rax = helper::handler_create("CreateDirectory2W");
    stdcall_ret(emu, 1);
}
