use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IntersectClipRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let left = api_try!(emu, call.arg_i32(emu, 1));
    let top = api_try!(emu, call.arg_i32(emu, 2));
    let right = api_try!(emu, call.arg_i32(emu, 3));
    let bottom = api_try!(emu, call.arg_i32(emu, 4));
    log_red!(
        emu,
        "gdi32!IntersectClipRect hdc:0x{:x} l:{} t:{} r:{} b:{}",
        hdc,
        left,
        top,
        right,
        bottom
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // SIMPLEREGION
}
