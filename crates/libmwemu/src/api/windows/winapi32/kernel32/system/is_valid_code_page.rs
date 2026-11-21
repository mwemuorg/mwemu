use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsValidCodePage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let codepage = api_try!(emu, call.arg_u32(emu, 0));

    log_red!(emu, "kernel32!IsValidCodePage {}", codepage);

    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}
