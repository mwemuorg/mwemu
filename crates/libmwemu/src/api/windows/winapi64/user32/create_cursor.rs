use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateCursor(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let x_hotspot = api_try!(emu, call.arg_i32(emu, 1));
    let y_hotspot = api_try!(emu, call.arg_i32(emu, 2));
    let width = api_try!(emu, call.arg_i32(emu, 3));
    let height = api_try!(emu, call.arg_i32(emu, 4));
    let and_plane = api_try!(emu, call.arg(emu, 5));
    let xor_plane = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "user32!CreateCursor hinst:0x{:x} hot:({},{}) size:{}x{}",
        hinstance,
        x_hotspot,
        y_hotspot,
        width,
        height
    );
    let hndl = helper::handler_create("user32://CreateCursor");
    call.set_return(emu, hndl);
}
