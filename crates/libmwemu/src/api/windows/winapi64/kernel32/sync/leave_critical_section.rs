use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LeaveCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cs_ptr = api_try!(emu, call.arg(emu, 0));
    let tid = emu.current_thread().id;

    log_red!(
        emu,
        "kernel32!LeaveCriticalSection thread: 0x{:x} cs: 0x{:x}",
        tid,
        cs_ptr
    );
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }

    // Release the lock and get any thread that should be woken up
    if let Some(wake_tid) = emu.global_locks.leave(cs_ptr, tid) {
        // Find the thread to wake up and clear its blocked state
        for thread in &mut emu.threads {
            if thread.id == wake_tid {
                thread.blocked_on_cs = None;
                thread.wake_tick = 0; // Make it runnable immediately
                log::trace!("  Waking thread 0x{:x}", wake_tid);
                break;
            }
        }
    }

    // Small delay to simulate atomic operation overhead
    helper::advance_tick(emu, 1);
}
