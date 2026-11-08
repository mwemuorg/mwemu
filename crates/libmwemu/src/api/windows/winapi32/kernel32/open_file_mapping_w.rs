use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn OpenFileMappingW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenFileMappingW");
    emu.regs_mut().rax = helper::handler_create("OpenFileMappingW");
    stdcall_ret(emu, 1);
}
