use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetCurrentProcessId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetCurrentProcessId");

    call.set_return(emu, 0x123);
}
