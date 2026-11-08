use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateFileMappingFromApp(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileMappingFromApp");
    emu.regs_mut().rax = helper::handler_create("CreateFileMappingFromApp");
    stdcall_ret(emu, 1);
}
