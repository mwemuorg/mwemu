use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ScrollWindow(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let x_amount = api_try!(emu, call.arg_i32(emu, 1));
    let y_amount = api_try!(emu, call.arg_i32(emu, 2));
    let lp_rect = api_try!(emu, call.arg(emu, 3));
    let lp_clip_rect = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "user32!ScrollWindow hwnd:0x{:x} dx:{} dy:{} rect:0x{:x} clip:0x{:x}",
        hwnd,
        x_amount,
        y_amount,
        lp_rect,
        lp_clip_rect
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
