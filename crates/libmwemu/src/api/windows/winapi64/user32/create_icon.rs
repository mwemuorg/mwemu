use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateIcon(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hinstance = api_try!(emu, call.arg(emu, 0));
    let width = api_try!(emu, call.arg_i32(emu, 1));
    let height = api_try!(emu, call.arg_i32(emu, 2));
    let planes = api_try!(emu, call.arg_u8(emu, 3));
    let bits_pixel = api_try!(emu, call.arg_u8(emu, 4));
    let and_bits = api_try!(emu, call.arg(emu, 5));
    let xor_bits = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "user32!CreateIcon hinst:0x{:x} size:{}x{} planes:{} bpp:{}",
        hinstance,
        width,
        height,
        planes,
        bits_pixel
    );
    let hndl = helper::handler_create("user32://CreateIcon");
    call.set_return(emu, hndl);
}
