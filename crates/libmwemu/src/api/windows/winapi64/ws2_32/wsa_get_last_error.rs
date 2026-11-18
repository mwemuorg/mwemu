use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn WSAGetLastError(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err = emu.get_last_error();
    log_red!(emu, "ws2_32!WSAGetLastError = {}", err);
    helper::advance_tick(emu, 1);
    call.set_return(emu, err);
}
