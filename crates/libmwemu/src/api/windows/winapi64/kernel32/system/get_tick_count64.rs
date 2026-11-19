use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let tick = helper::read_tick_count(emu);
    log_red!(emu, "kernel32!GetTickCount64 -> {}", tick);
    call.set_return(emu, tick);
}
