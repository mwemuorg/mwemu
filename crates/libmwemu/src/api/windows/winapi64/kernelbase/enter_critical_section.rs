use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn EnterCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cs_ptr = api_try!(emu, call.arg(emu, 0));
    helper::advance_tick(emu, 1);
    log_red!(emu, "kernelbase!EnterCriticalSection 0x{:x}", cs_ptr);
    let handle = helper::handler_create("EnterCriticalSection");
}
