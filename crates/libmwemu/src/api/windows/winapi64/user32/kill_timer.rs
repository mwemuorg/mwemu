use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn KillTimer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let timer_id = api_try!(emu, call.arg(emu, 1));
    helper::handler_close(timer_id);
    log_red!(emu, "user32!KillTimer hwnd:0x{:x} timer:{}", hwnd, timer_id);
    call.set_return(emu, 1);
}
