use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryUnbiasedInterruptTime(emu: &mut emu::Emu) {
    let p_unbiased_time = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!QueryUnbiasedInterruptTime pTime:0x{:x}",
        p_unbiased_time
    );
    if p_unbiased_time != 0 {
        emu.maps.write_qword(p_unbiased_time, emu.tick as u64);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
