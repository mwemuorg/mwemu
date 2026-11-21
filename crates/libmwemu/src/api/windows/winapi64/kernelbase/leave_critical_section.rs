use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LeaveCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cs_ptr = api_try!(emu, call.arg(emu, 0));
    helper::advance_tick(emu, 1);
    log_red!(emu, "kernelbase!LeaveCriticalSection 0x{:x}", cs_ptr);
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
}
