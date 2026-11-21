use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
pub fn GlobalLock(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let mem = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!GlobalLock mem: 0x{:x}", mem);
    let handle = helper::handler_create("GlobalLock");
    emu.set_last_error(0);
    call.set_return(emu, mem);
}
