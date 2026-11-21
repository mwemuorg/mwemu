use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn EnterCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cs_ptr = api_try!(emu, call.arg(emu, 0));
    let tid = emu.current_thread().id;

    log_red!(
        emu,
        "kernel32!EnterCriticalSection thread: 0x{:x} cs: 0x{:x}",
        tid,
        cs_ptr
    );
    let handle = helper::handler_create("EnterCriticalSection");

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
