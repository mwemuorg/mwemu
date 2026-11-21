use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenThreadDesktop(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let hndl = helper::handler_create("OpenThreadDesktop");
    log_red!(emu, "user32!OpenThreadDesktop => 0x{:x}", hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
