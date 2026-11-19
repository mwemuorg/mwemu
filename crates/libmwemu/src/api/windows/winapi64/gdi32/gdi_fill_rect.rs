use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GdiFillRect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let rect = api_try!(emu, call.arg(emu, 1));
    let brush = api_try!(emu, call.arg(emu, 2));
    let (left, top, right, bottom) = if rect != 0 {
        (
            emu.maps.read_dword(rect).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 4).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 8).unwrap_or(0) as i32,
            emu.maps.read_dword(rect + 12).unwrap_or(0) as i32,
        )
    } else {
        (0, 0, 0, 0)
    };
    log_red!(
        emu,
        "gdi32!FillRect hdc:0x{:x} rect:({},{},{},{}) brush:0x{:x}",
        hdc,
        left,
        top,
        right,
        bottom,
        brush
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
