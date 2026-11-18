use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn AllocConsole(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!AllocConsole arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    let handle = helper::handler_create("allocconsole");
    call.set_return(emu, handle);
}
