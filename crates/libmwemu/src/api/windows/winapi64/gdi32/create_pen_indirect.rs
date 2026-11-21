use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreatePenIndirect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lpen = api_try!(emu, call.arg(emu, 0));
    let (style, width, color) = if lpen != 0 {
        (
            emu.maps.read_dword(lpen).unwrap_or(0),
            emu.maps.read_dword(lpen + 4).unwrap_or(0) as i32,
            emu.maps.read_dword(lpen + 12).unwrap_or(0),
        )
    } else {
        (0, 0, 0)
    };
    log_red!(
        emu,
        "gdi32!CreatePenIndirect lpen:0x{:x} style:{} width:{} color:0x{:06x}",
        lpen,
        style,
        width,
        color
    );
    call.set_return(emu, helper::handler_create("gdi32://pen"));
}
