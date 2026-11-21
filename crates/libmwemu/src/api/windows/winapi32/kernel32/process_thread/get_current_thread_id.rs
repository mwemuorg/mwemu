use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetCurrentThreadId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetCurrentThreadId");

    call.set_return(emu, 0x111); //TODO: track pids and tids
}
