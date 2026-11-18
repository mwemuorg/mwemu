use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetTimer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let id = api_try!(emu, call.arg(emu, 1));
    let elapse = api_try!(emu, call.arg_u32(emu, 2));
    let callback = api_try!(emu, call.arg(emu, 3));
    let timer_handle = helper::handler_create("timer");
    log_red!(
        emu,
        "user32!SetTimer hwnd:0x{:x} id:{} ms:{} cb:0x{:x} => 0x{:x}",
        hwnd,
        id,
        elapse,
        callback,
        timer_handle
    );
    emu.set_last_error(0);
    call.set_return(emu, timer_handle);
}
