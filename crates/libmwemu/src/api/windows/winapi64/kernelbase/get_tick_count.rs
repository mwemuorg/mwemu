use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let tick = helper::read_tick_count(emu) as u32 as u64;
    log_red!(emu, "kernelbase!GetTickCount => {}", tick);
    emu.set_last_error(0);
    call.set_return(emu, tick);
}
