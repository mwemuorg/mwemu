use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryPerformanceFrequency(emu: &mut emu::Emu) {
    let out_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!QueryPerformanceFrequency");
    emu.maps.write_qword(out_ptr, 10_000_000);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
