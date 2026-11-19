use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn IsWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let window = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "user32!IsWindow hwnd:0x{:x}", window);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}
