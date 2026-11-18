use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn IsDebuggerPresent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!IsDebuggerPresent");
    call.set_return(emu, 0); // of course :p
}
