use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn TryEnterCriticalSection(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cs_ptr = api_try!(emu, call.arg(emu, 0));
    helper::advance_tick(emu, 1);
    log_red!(emu, "kernelbase!TryEnterCriticalSection 0x{:x}", cs_ptr);
    let handle = helper::handler_create("TryEnterCriticalSection");
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
