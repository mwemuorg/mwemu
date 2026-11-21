use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SetFocus(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "user32!SetFocus hwnd:0x{:x}", hwnd);
    call.set_return(emu, hwnd as u64);
    call.cleanup_args(emu, 1);
}
