use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn MoveWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let x = api_try!(emu, call.arg_i32(emu, 1));
    let y = api_try!(emu, call.arg_i32(emu, 2));
    let width = api_try!(emu, call.arg_i32(emu, 3));
    let height = api_try!(emu, call.arg_i32(emu, 4));
    let repaint = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "user32!MoveWindow hwnd:0x{:x} x:{} y:{} w:{} h:{} repaint:{}",
        hwnd,
        x,
        y,
        width,
        height,
        repaint
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
