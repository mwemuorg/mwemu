use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn atexit(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let fptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!atexit fptr: 0x{:x}", fptr);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
