use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateFontIndirectA(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectA");
}

fn create_font_indirect(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(emu, "gdi32!{api} arg1:0x{:x}", arg1);
    call.cleanup_args(emu, 1);
    let hndl = helper::handler_create(&format!("gdi32://{api}"));
    assert!(hndl < 0x00000001_00000000);
    call.set_return(emu, hndl);
}

pub fn CreateFontIndirectW(emu: &mut emu::Emu) {
    create_font_indirect(emu, "CreateFontIndirectW");
}
