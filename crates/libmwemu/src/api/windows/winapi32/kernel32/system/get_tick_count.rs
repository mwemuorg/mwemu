use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;

pub fn GetTickCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetTickCount");

    let return_value = emu.tick as u64;
    call.set_return(emu, return_value);
}
