use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn OleBuildVersion(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "ole32!OleBuildVersion");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, (23u64 << 16) | 639);
}
