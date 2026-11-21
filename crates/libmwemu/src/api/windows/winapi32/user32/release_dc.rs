use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn ReleaseDC(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg_u32(emu, 0));
    let hdc = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "user32!ReleaseDC hwnd: 0x{:x} hdc: 0x{:x}", hwnd, hdc);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
