use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateBrushIndirect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lbrush = api_try!(emu, call.arg(emu, 0));
    let (style, color) = if lbrush != 0 {
        (
            emu.maps.read_dword(lbrush).unwrap_or(0),
            emu.maps.read_dword(lbrush + 4).unwrap_or(0),
        )
    } else {
        (0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreateBrushIndirect lbrush:0x{:x} style:{} color:0x{:06x}",
        lbrush,
        style,
        color
    );
    call.set_return(emu, helper::handler_create("gdi32://brush"));
}
