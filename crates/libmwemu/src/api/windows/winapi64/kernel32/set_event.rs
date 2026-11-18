use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetEvent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!SetEvent handle: 0x{:x}", handle);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
