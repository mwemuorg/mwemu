use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WaitForDebugEventEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "kernel32!WaitForDebugEventEx 0x{:x}", arg1);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
