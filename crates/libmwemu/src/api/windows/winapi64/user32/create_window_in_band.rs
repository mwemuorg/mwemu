use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateWindowInBand(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("CreateWindowInBand");
    log_red!(emu, "user32!CreateWindowInBand => 0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
