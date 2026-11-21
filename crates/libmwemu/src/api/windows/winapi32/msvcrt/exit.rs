use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn exit(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let status = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "msvcrt!exit {}", status);
    emu.stop();
}
