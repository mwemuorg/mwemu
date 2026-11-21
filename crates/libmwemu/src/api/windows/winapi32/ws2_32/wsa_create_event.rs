use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn WSACreateEvent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let event = helper::handler_create("wsa_event");
    log_red!(emu, "ws2_32!WSACreateEvent -> 0x{:x}", event);
    call.set_return(emu, event);
}
