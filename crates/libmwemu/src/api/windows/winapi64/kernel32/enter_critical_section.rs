use crate::emu;
use crate::winapi::helper;

pub fn EnterCriticalSection(emu: &mut emu::Emu) {
    let cs_ptr = emu.regs().rcx;
    let tid = emu.current_thread().id;

    log_red!(
        emu,
        "kernel32!EnterCriticalSection thread: 0x{:x} cs: 0x{:x}",
        tid,
        cs_ptr
    );

    let acquired = emu.global_locks.enter(cs_ptr, tid);

    if acquired {
        // Lock acquired immediately
        // Small delay to simulate atomic operation overhead
        helper::advance_tick(emu, 1);
    } else {
        // Thread is blocked — mark it as waiting
        let thread_idx = emu.current_thread_id;
        emu.threads[thread_idx].blocked_on_cs = Some(cs_ptr);
        // Don't set wake_tick here - it will be cleared when lock is released

        // Simulate the wait by advancing tick slightly
        // The actual blocking will be handled by the scheduler
        helper::advance_tick(emu, 2);
    }
}
