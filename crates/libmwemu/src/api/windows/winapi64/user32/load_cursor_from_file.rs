use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn LoadCursorFromFileA(emu: &mut emu::Emu) {
    load_cursor_from_file(emu, "LoadCursorFromFileA");
}

fn load_cursor_from_file(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!{api} arg1:0x{:x}", arg1);
    let hndl = helper::handler_create(&format!("user32://{api}"));
    call.set_return(emu, hndl);
}

pub fn LoadCursorFromFileW(emu: &mut emu::Emu) {
    load_cursor_from_file(emu, "LoadCursorFromFileW");
}
