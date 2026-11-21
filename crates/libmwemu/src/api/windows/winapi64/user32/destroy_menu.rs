use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn DestroyMenu(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmenu = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!DestroyMenu hmenu:0x{:x}", hmenu);
    if hmenu != 0 && helper::handler_exist(hmenu) {
        helper::handler_close(hmenu);
    }
    call.set_return(emu, 1);
}
