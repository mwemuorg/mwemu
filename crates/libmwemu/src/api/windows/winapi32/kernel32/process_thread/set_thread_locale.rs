use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetThreadLocale(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernel32!SetThreadLocale {}", locale);

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}
