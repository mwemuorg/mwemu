use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;

pub fn socket(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "ws2_32!socket");

    call.cleanup_args(emu, 3);

    call.set_return(emu, helper::socket_create());
}
