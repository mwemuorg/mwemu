use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OleIsRunning(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pole_obj = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!OleIsRunning pOleObj:0x{:x}", pole_obj);
    let running = if helper::handler_exist(pole_obj) {
        1u64
    } else {
        0u64
    };
    emu.set_last_error(0);
    call.set_return(emu, running);
}
