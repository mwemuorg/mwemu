use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn GetTickCount64(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let tick = helper::read_tick_count(emu);
    log_red!(emu, "kernelbase!GetTickCount64 => {}", tick);
    emu.set_last_error(0);
    call.set_return(emu, tick);
}
