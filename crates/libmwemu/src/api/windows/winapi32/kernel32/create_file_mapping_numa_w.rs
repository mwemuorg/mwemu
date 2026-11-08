use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateFileMappingNumaW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateFileMappingNumaW");
    emu.regs_mut().rax = helper::handler_create("CreateFileMappingNumaW");
    stdcall_ret(emu, 1);
}
