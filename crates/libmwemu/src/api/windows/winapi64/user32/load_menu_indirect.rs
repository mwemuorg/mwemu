use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadMenuIndirectA(emu: &mut emu::Emu) {
    load_menu_indirect(emu, "LoadMenuIndirectA");
}

fn load_menu_indirect(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let template = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!{api} tmpl:0x{:x}", template);
    let hndl = helper::handler_create("user32://menu");
    call.set_return(emu, hndl);
}

pub fn LoadMenuIndirectW(emu: &mut emu::Emu) {
    load_menu_indirect(emu, "LoadMenuIndirectW");
}
