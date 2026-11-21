use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ResetEvent(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hEvent = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!ResetEvent 0x{:x}", hEvent);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
