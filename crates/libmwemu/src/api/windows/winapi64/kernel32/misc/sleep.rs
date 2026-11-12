use crate::emu;
use crate::winapi::helper;

pub fn Sleep(emu: &mut emu::Emu) {
    let millis = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!Sleep thread: 0x{:x} millis: {}{}",
        emu.current_thread().id,
        millis,
        if emu.cfg.short_circuit_sleep {
            " [short-circuited]"
        } else {
            ""
        }
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    if emu.cfg.short_circuit_sleep {
        // Short-circuit: advance tick but return immediately
        emu.tick += 1;
        return;
    }

    if millis > 0 {
        // Set wake tick for this thread
        let thread_idx = emu.current_thread_id;
        emu.threads[thread_idx].wake_tick = emu.tick + millis as usize;
    }

    // Advance global tick
    helper::advance_tick(emu, millis);
}
