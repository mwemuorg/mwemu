use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CoIsHandlerConnected(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let punk = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoIsHandlerConnected punk:0x{:x}", punk);
    let connected = if helper::handler_exist(punk) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    call.set_return(emu, connected);
}
