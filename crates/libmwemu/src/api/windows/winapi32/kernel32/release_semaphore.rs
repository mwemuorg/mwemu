use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ReleaseSemaphore(emu: &mut emu::Emu) {
    let h_semaphore = arg(emu, 0) as u64;
    let release_count = arg(emu, 1) as u64;
    let lp_previous_count = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!ReleaseSemaphore hSemaphore:0x{:x} releaseCount:{} lpPrevCount:0x{:x}",
        h_semaphore,
        release_count,
        lp_previous_count
    );

    // Write previous count if output pointer is non-null
    if lp_previous_count != 0 {
        emu.maps.write_dword(lp_previous_count, 1); // previous count = 1
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 3);
}
