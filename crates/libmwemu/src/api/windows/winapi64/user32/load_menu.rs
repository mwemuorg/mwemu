use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadMenuA(emu: &mut emu::Emu) {
    load_menu(emu, "LoadMenuA");
}

fn load_menu(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let hinst = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!{api} hInst:0x{:x}", hinst);
    let hndl = helper::handler_create("user32://menu");
    call.set_return(emu, hndl);
}

pub fn LoadMenuW(emu: &mut emu::Emu) {
    load_menu(emu, "LoadMenuW");
}
