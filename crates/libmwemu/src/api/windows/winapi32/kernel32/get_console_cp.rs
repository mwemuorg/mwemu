use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetConsoleCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleCP");
    emu.regs_mut().rax = 0x409;
    stdcall_ret(emu, 0);
}
