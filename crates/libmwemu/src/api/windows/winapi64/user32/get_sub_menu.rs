use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetSubMenu(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmenu = api_try!(emu, call.arg(emu, 0));
    let npos = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "user32!GetSubMenu menu:0x{:x} pos:{}", hmenu, npos);
    let hndl = helper::handler_create("user32://menu");
    call.set_return(emu, hndl);
}
