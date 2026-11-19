use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetThreadId(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!GetThreadId h:0x{:x}", handle);
    if helper::handler_exist(handle) {
        call.set_return(emu, 0x2345);
    } else {
        call.set_return(emu, 0);
    }
    emu.set_last_error(0);
}
