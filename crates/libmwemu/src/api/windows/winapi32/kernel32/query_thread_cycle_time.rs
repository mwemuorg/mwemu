use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn QueryThreadCycleTime(emu: &mut emu::Emu) {
    let h_thread = arg(emu, 0) as u64;
    let p_cycle_time = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!QueryThreadCycleTime hThread:0x{:x} pCycleTime:0x{:x}",
        h_thread,
        p_cycle_time
    );
    if p_cycle_time != 0 {
        emu.maps.write_qword(p_cycle_time, emu.tick as u64);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
