use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetPixelGdi(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let x = api_try!(emu, call.arg_i32(emu, 1));
    let y = api_try!(emu, call.arg_i32(emu, 2));
    log_red!(emu, "gdi32!GetPixel hdc:0x{:x} x:{} y:{}", hdc, x, y);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    call.set_return(emu, 0); // black pixel
}
