use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetConsoleCP(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!GetConsoleCP");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 437);
}
