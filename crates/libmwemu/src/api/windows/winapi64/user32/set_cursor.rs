use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SetCursor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hcursor = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!SetCursor hCursor:0x{:x}", hcursor);
    call.set_return(emu, hcursor);
}
