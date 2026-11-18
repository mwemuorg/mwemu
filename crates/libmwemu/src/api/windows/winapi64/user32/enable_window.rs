use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn EnableWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let argument_0 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!EnableWindow hwnd:0x{:x}", argument_0);
    call.set_return(emu, 0);
}
