use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetMenuItemCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hmenu = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!GetMenuItemCount hmenu:0x{:x}", hmenu);
    let count = if helper::handler_exist(hmenu) {
        1u64
    } else {
        0xffffffffu64
    };
    emu.set_last_error(0);
    call.set_return(emu, count);
}
