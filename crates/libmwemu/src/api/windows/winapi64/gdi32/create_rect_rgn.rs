use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateRectRgn(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let x1 = api_try!(emu, call.arg_i32(emu, 0));
    let y1 = api_try!(emu, call.arg_i32(emu, 1));
    let x2 = api_try!(emu, call.arg_i32(emu, 2));
    let y2 = api_try!(emu, call.arg_i32(emu, 3));
    log_red!(
        emu,
        "gdi32!CreateRectRgn x1:{} y1:{} x2:{} y2:{}",
        x1,
        y1,
        x2,
        y2
    );
    call.set_return(emu, helper::handler_create("gdi32://region"));
}
