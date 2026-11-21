use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateIconIndirect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let iconinfo_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "user32!CreateIconIndirect info:0x{:x}", iconinfo_ptr);
    let hndl = helper::handler_create("user32://CreateIconIndirect");
    call.set_return(emu, hndl);
}
