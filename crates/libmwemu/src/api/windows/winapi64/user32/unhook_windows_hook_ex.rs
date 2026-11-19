use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn UnhookWindowsHookEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hhk = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!UnhookWindowsHookEx hhk:0x{:x}", hhk);
    if hhk != 0 {
        helper::handler_close(hhk);
    }
    call.set_return(emu, 1);
}
