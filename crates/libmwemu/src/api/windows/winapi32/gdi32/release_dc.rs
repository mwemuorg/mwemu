use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ReleaseDC(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg_u32(emu, 0));
    let hdc = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "gdi32!ReleaseDC hwnd:0x{:x} hdc:0x{:x}", hwnd, hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
