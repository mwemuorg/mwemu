use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetThreadLocale(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetThreadLocale");
    emu.regs_mut().rax = 0x409; // English (United States)
    stdcall_ret(emu, 0);
}
