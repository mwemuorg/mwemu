use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GdiPie(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let left = api_try!(emu, call.arg_i32(emu, 1));
    let top = api_try!(emu, call.arg_i32(emu, 2));
    let right = api_try!(emu, call.arg_i32(emu, 3));
    let bottom = api_try!(emu, call.arg_i32(emu, 4));
    let x_start = api_try!(emu, call.arg_i32(emu, 5));
    let y_start = api_try!(emu, call.arg_i32(emu, 6));
    let x_end = api_try!(emu, call.arg_i32(emu, 7));
    let y_end = api_try!(emu, call.arg_i32(emu, 8));
    log_red!(
        emu,
        "gdi32!Pie hdc:0x{:x} rect:({},{},{},{}) start:({},{}) end:({},{})",
        hdc,
        left,
        top,
        right,
        bottom,
        x_start,
        y_start,
        x_end,
        y_end
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
