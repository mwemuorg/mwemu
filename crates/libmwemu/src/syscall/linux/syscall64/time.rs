//! Clock syscalls backed by the host clock: programs that sleep, time out or
//! schedule timers (Go's runtime does all three) need `nanotime()` to move.

use crate::emu;
use crate::windows::constants;
use std::time::{SystemTime, UNIX_EPOCH};

const CLOCK_REALTIME: u64 = 0;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_CLOCK_GETTIME => clock_gettime(emu),
        constants::NR64_GETTIMEOFDAY => gettimeofday(emu),
        _ => return false,
    }
    true
}

/// Seconds and nanoseconds for `clock_id`: wall clock for REALTIME, the
/// emulator's own monotonic clock for everything else.
fn now(emu: &emu::Emu, clock_id: u64) -> (u64, u64) {
    let d = if clock_id == CLOCK_REALTIME {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
    } else {
        emu.now.elapsed()
    };
    (d.as_secs(), d.subsec_nanos() as u64)
}

/// clock_gettime(clock_id, *timespec)
fn clock_gettime(emu: &mut emu::Emu) {
    let clock_id = emu.regs().rdi;
    let ts = emu.regs().rsi;
    let (sec, nsec) = now(emu, clock_id);
    emu.maps.write_qword(ts, sec);
    emu.maps.write_qword(ts + 8, nsec);
    emu.regs_mut().rax = 0;
    super::misc::trace_syscall64_args(
        emu,
        "clock_gettime",
        &[("clock", clock_id.to_string()), ("sec", sec.to_string())],
    );
}

/// gettimeofday(*timeval, *timezone)
fn gettimeofday(emu: &mut emu::Emu) {
    let tv = emu.regs().rdi;
    let (sec, nsec) = now(emu, CLOCK_REALTIME);
    if tv != 0 {
        emu.maps.write_qword(tv, sec);
        emu.maps.write_qword(tv + 8, nsec / 1000);
    }
    emu.regs_mut().rax = 0;
    super::misc::trace_syscall64_args(emu, "gettimeofday", &[("sec", sec.to_string())]);
}
