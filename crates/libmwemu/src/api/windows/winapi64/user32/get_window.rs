use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let cmd = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!GetWindow hwnd:0x{:x} cmd:{}", hwnd, cmd);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
