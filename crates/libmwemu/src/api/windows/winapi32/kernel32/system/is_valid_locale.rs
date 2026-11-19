use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsValidLocale(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg_u32(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!IsValidLocale");

    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
