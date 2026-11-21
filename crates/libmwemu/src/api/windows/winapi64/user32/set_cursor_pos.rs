use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn SetCursorPos(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let x = api_try!(emu, call.arg(emu, 0));
    let y = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!SetCursorPos x:{} y:{}", x, y);
    call.set_return(emu, 1);
}
