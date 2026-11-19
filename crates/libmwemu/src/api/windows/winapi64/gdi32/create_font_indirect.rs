use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateFontIndirectA(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectA");
}

fn create_font_indirect(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let lf = api_try!(emu, call.arg(emu, 0));
    let height = if lf != 0 {
        emu.maps.read_dword(lf).unwrap_or(0) as i32
    } else {
        0
    };
    log_red!(emu, "gdi32!{api} lf:0x{:x} height:{}", lf, height);
    call.set_return(emu, helper::handler_create("gdi32://font"));
}

pub fn CreateFontIndirectW(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectW");
}
