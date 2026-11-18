use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateRectRgnIndirect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rect = api_try!(emu, call.arg(emu, 0));
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
        "gdi32!CreateRectRgnIndirect rect:({},{},{},{})",
        left,
        top,
        right,
        bottom
    );
    call.set_return(emu, helper::handler_create("gdi32://region"));
}
