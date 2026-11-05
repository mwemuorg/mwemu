use crate::emu;

pub fn ReleaseSemaphore(emu: &mut emu::Emu) {
    let h_semaphore = emu.regs().rcx;
    let release_count = emu.regs().rdx;
    let lp_previous_count = emu.regs().r8;

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
}
