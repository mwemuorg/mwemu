use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreatePen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let style = api_try!(emu, call.arg_u32(emu, 0));
    let width = api_try!(emu, call.arg_i32(emu, 1));
    let color = api_try!(emu, call.arg_u32(emu, 2));
    let style_name = match style {
        0 => "PS_SOLID",
        1 => "PS_DASH",
        2 => "PS_DOT",
        3 => "PS_DASHDOT",
        4 => "PS_DASHDOTDOT",
        5 => "PS_NULL",
        6 => "PS_INSIDEFRAME",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!CreatePen style:{} ({}) width:{} color:0x{:06x}",
        style,
        style_name,
        width,
        color
    );
    call.set_return(emu, helper::handler_create("gdi32://pen"));
}
