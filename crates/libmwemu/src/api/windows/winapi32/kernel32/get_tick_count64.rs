use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetTickCount64");
    // ULONGLONG comes back in edx:eax
    let tick = emu.tick as u64;
    emu.regs_mut().rax = tick & 0xffff_ffff;
    emu.regs_mut().rdx = tick >> 32;
    stdcall_ret(emu, 0);
}
