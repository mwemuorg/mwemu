use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetCurrentThreadId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let tid = emu.current_thread().id;
    log_red!(emu, "kernel32!GetCurrentThreadId -> 0x{:x}", tid);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, tid);
}
