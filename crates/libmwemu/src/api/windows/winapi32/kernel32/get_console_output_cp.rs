use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetConsoleOutputCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleOutputCP");
    emu.regs_mut().rax = 0x409;
    stdcall_ret(emu, 0);
}
