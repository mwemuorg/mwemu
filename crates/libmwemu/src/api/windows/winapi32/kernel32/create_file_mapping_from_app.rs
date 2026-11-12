use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateFileMappingFromApp(emu: &mut emu::Emu) {
    stdcall_ret(emu, 6);
    log_red!(emu, "kernel32!CreateFileMappingFromApp");
    emu.regs_mut().rax = helper::handler_create("CreateFileMappingFromApp");
}
