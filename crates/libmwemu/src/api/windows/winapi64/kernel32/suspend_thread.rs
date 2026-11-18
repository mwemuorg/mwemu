use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SuspendThread(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!SuspendThread h:0x{:x}", handle);
    emu.set_last_error(0);
    call.set_return(emu, 0); // previous suspend count
}
