use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetPriorityClass(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!GetPriorityClass");
    call.cleanup_args(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0x20);
}
